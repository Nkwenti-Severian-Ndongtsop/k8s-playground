use axum::{routing::{post, get}, Router, Json, extract::Extension, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use sqlx::Row;
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
struct RegisterInput {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    username: String,
}

async fn register(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<RegisterInput>,
) -> StatusCode {
    let password_hash = hash(&input.password, DEFAULT_COST).unwrap();
    let res = sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(&input.username)
        .bind(&password_hash)
        .execute(&pool)
        .await;
    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

async fn login(
    Extension(pool): Extension<PgPool>,
    Extension(secret): Extension<String>,
    Json(input): Json<LoginInput>,
) -> Result<Json<HashMap<&'static str, String>>, StatusCode> {
    let row = sqlx::query("SELECT password_hash FROM users WHERE username = $1")
        .bind(&input.username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(row) = row {
        let password_hash: String = row.get("password_hash");
        if verify(&input.password, &password_hash).unwrap_or(false) {
            // Create JWT
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;
            let claims = Claims {
                sub: input.username,
                exp: expiration,
            };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            let mut resp = HashMap::new();
            resp.insert("token", token);
            return Ok(Json(resp));
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

async fn me(
    Extension(secret): Extension<String>,
    Json(token): Json<HashMap<String, String>>,
) -> Result<Json<UserResponse>, StatusCode> {
    if let Some(token) = token.get("token") {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ).map_err(|_| StatusCode::UNAUTHORIZED)?;
        let username = token_data.claims.sub;
        return Ok(Json(UserResponse { username }));
    }
    Err(StatusCode::UNAUTHORIZED)
}

#[tokio::main]
async fn main() {
    println!("Starting backend server...");
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", post(me))
        .layer(Extension(pool))
        .layer(Extension(jwt_secret))
        .route("/health", get(|| async { "backend k8s-net is running\n" }))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Backend server running on port 8080");
    axum::serve(listener, app.into_make_service()).await.unwrap();
} 