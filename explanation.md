# Kubernetes Communication Architecture Explanation

## Overview
This project implements a full-stack web application with frontend, backend, and database components deployed on Kubernetes. The architecture uses LoadBalancer services for external access and internal services for inter-pod communication.

## Namespace: `kube-net`
All components are deployed in the `kube-net` namespace for proper isolation and service discovery.

## Component Architecture

### 1. Frontend (React/HTML/CSS/JS)
- **Image**: `nkwentihub/k8s-frontend:latest`
- **Port**: 80 (container), 30080 (NodePort), 80 (LoadBalancer)
- **Purpose**: User interface for login, registration, and profile management

### 2. Backend (Rust/Axum)
- **Image**: `nkwentihub/k8s-backend:latest`
- **Port**: 8080 (container), 30081 (LoadBalancer)
- **Purpose**: REST API server handling authentication and user management

### 3. Database (PostgreSQL)
- **Image**: `postgres:15`
- **Port**: 5432 (container and service)
- **Purpose**: Persistent data storage for users and authentication

## Service Communication Matrix

### External Access Points (LoadBalancer Services)

#### Frontend LoadBalancer (`kube-frontend-lb`)
- **External Port**: 80
- **Target Port**: 80
- **Access URL**: `http://<vm-ip>:80`
- **Purpose**: Main entry point for web application

#### Backend LoadBalancer (`kube-backend-lb`)
- **External Port**: 30081
- **Target Port**: 8080
- **Access URL**: `http://<vm-ip>:30081`
- **Purpose**: Direct API access (if needed)

### Internal Service Communication

#### Frontend → Backend
- **Connection**: Frontend JavaScript → Backend LoadBalancer
- **Protocol**: HTTP/HTTPS
- **Port**: 30081
- **Endpoints**:
  - `POST /register` - User registration
  - `POST /login` - User authentication
  - `POST /me` - Get user profile
  - `GET /health` - Health check

#### Backend → Database
- **Connection**: Backend container → PostgreSQL service
- **Protocol**: PostgreSQL
- **Port**: 5432
- **Database**: `kubenet_db`
- **User**: `nkwenti`
- **Password**: `mysecretpassword`

### Internal Kubernetes Services

#### PostgreSQL Service (`postgres`)
- **Type**: ClusterIP
- **Port**: 5432
- **Purpose**: Internal database access for backend
- **Connection String**: `postgres://nkwenti:mysecretpassword@postgres:5432/kubenet_db`

#### Frontend Service (`kube-frontend`)
- **Type**: NodePort
- **Port**: 80
- **NodePort**: 30080
- **Purpose**: Internal frontend access

#### Backend Service (`kube-backend`)
- **Type**: ClusterIP
- **Port**: 8080
- **Purpose**: Internal backend access

## API Endpoints

### Authentication Endpoints
1. **POST /register**
   - **Purpose**: Create new user account
   - **Request Body**: `{"username": "string", "password": "string"}`
   - **Response**: 201 (Created) or 409 (Conflict)
   - **Access**: Frontend → Backend LoadBalancer

2. **POST /login**
   - **Purpose**: Authenticate user and get JWT token
   - **Request Body**: `{"username": "string", "password": "string"}`
   - **Response**: `{"token": "jwt_token"}` or 401 (Unauthorized)
   - **Access**: Frontend → Backend LoadBalancer

3. **POST /me**
   - **Purpose**: Get current user profile using JWT token
   - **Request Body**: `{"token": "jwt_token"}`
   - **Response**: `{"username": "string"}` or 401 (Unauthorized)
   - **Access**: Frontend → Backend LoadBalancer

### Health Check Endpoint
4. **GET /health**
   - **Purpose**: Health check for Kubernetes probes
   - **Response**: `"backend k8s-net is running\n"`
   - **Access**: Kubernetes → Backend container

## Environment Variables

### Backend Environment Variables
- **DATABASE_URL**: `postgres://nkwenti:mysecretpassword@postgres:5432/kubenet_db`
  - **Source**: `backend-config` ConfigMap
  - **Purpose**: Database connection string

- **JWT_SECRET**: `eW91cl9zdXBlcl9zZWNyZXRfa2V5Cg==` (base64 encoded)
  - **Source**: `backend-secret` Secret
  - **Purpose**: JWT token signing key

### PostgreSQL Environment Variables
- **POSTGRES_DB**: `kubenet_db`
  - **Source**: `postgres-config` ConfigMap
  - **Purpose**: Database name

- **POSTGRES_USER**: `nkwenti`
  - **Source**: `postgres-config` ConfigMap
  - **Purpose**: Database user

- **POSTGRES_PASSWORD**: `bXlzZWNyZXRwYXNzd29yZA==` (base64 encoded `mysecretpassword`)
  - **Source**: `postgres-secret` Secret
  - **Purpose**: Database password

## Network Flow

### User Registration Flow
1. User accesses frontend at `http://<vm-ip>:80`
2. User fills registration form
3. Frontend JavaScript sends POST to `http://<vm-ip>:30081/register`
4. Backend receives request and connects to PostgreSQL
5. Backend creates user record in database
6. Backend returns 201 status to frontend
7. Frontend redirects to login page

### User Login Flow
1. User accesses frontend at `http://<vm-ip>:80`
2. User fills login form
3. Frontend JavaScript sends POST to `http://<vm-ip>:30081/login`
4. Backend receives request and connects to PostgreSQL
5. Backend verifies credentials and generates JWT token
6. Backend returns JWT token to frontend
7. Frontend stores token and redirects to profile

### Profile Access Flow
1. User accesses profile page
2. Frontend JavaScript sends POST to `http://<vm-ip>:30081/me` with JWT token
3. Backend validates JWT token
4. Backend returns user profile data
5. Frontend displays user information

## Security Considerations

### CORS Configuration
- Backend uses `CorsLayer::permissive()` to allow cross-origin requests
- Frontend connects to backend using dynamic hostname resolution

### Authentication
- Passwords are hashed using bcrypt
- JWT tokens are used for session management
- Tokens expire after 24 hours

### Database Security
- Database credentials are stored in Kubernetes Secrets
- Database is only accessible within the cluster
- No external database access

## Monitoring and Health Checks

### Readiness Probes
- Backend has readiness probe on `/health` endpoint
- Ensures backend is ready to receive traffic

### Logging
- Backend logs startup messages and connection status
- Database logs connection attempts and errors

## Scaling Considerations

### Horizontal Scaling
- Frontend and backend deployments can be scaled independently
- Database uses persistent storage for data retention

### Load Balancing
- LoadBalancer services distribute traffic across multiple pods
- NodePort services provide alternative access methods

## Troubleshooting Connections

### Common Issues
1. **CORS Errors**: Ensure frontend connects to correct backend port (30081)
2. **Database Connection**: Verify PostgreSQL service is running and accessible
3. **JWT Issues**: Check JWT_SECRET environment variable
4. **Namespace Issues**: Ensure all components are in `kube-net` namespace

### Debug Commands
```bash
# Check pod status
kubectl get pods -n kube-net

# Check service endpoints
kubectl get svc -n kube-net

# Check logs
kubectl logs -f deployment/kube-backend -n kube-net
kubectl logs -f deployment/kube-frontend -n kube-net

# Access postgres
kubectl exec -it <postgres-pod> -n kube-net -- psql -U nkwenti -d kubenet_db
```

## 🔧 Configurable Values

Below are the values you can customize for your own deployment. These are the current values used in this project, but you can modify them according to your needs.

### Namespace Configuration
```yaml
# Current: kube-net
# Change to: your-preferred-namespace
namespace: kube-net
```

### Database Configuration
```yaml
# Database Name
# Current: kubenet_db
# Change to: your_database_name
POSTGRES_DB: kubenet_db

# Database User
# Current: nkwenti
# Change to: your_database_user
POSTGRES_USER: nkwenti

# Database Password
# Current: mysecretpassword
# Change to: your_secure_password
POSTGRES_PASSWORD: mysecretpassword

# Database Connection String
# Current: postgres://nkwenti:mysecretpassword@postgres:5432/kubenet_db
# Change to: postgres://your_user:your_password@postgres:5432/your_database
DATABASE_URL: postgres://nkwenti:mysecretpassword@postgres:5432/kubenet_db
```

### JWT Configuration
```yaml
# JWT Secret (base64 encoded)
# Current: eW91cl9zdXBlcl9zZWNyZXRfa2V5Cg== (decodes to: your_super_secret_key)
# Change to: your_own_jwt_secret (encode with: echo -n "your_secret" | base64)
JWT_SECRET: eW91cl9zdXBlcl9zZWNyZXRfa2V5Cg==
```

### Port Configuration
```yaml
# Frontend Ports
# Current: 80 (container), 30080 (NodePort), 80 (LoadBalancer)
# Change to: your preferred ports
FRONTEND_CONTAINER_PORT: 80
FRONTEND_NODEPORT: 30080
FRONTEND_LOADBALANCER_PORT: 80

# Backend Ports
# Current: 8080 (container), 30081 (LoadBalancer)
# Change to: your preferred ports
BACKEND_CONTAINER_PORT: 8080
BACKEND_LOADBALANCER_PORT: 30081

# Database Port
# Current: 5432
# Change to: your preferred port (if needed)
POSTGRES_PORT: 5432
```

```

### Deployment Names
```yaml
# Frontend Deployment
# Current: kube-frontend
# Change to: your-frontend-deployment-name
FRONTEND_DEPLOYMENT: kube-frontend

# Backend Deployment
# Current: kube-backend
# Change to: your-backend-deployment-name
BACKEND_DEPLOYMENT: kube-backend

# Database Deployment
# Current: postgres
# Change to: your-database-deployment-name
POSTGRES_DEPLOYMENT: postgres
```

### Service Names
```yaml
# Frontend Services
# Current: kube-frontend, kube-frontend-lb
# Change to: your-frontend-service-names
FRONTEND_SERVICE: kube-frontend
FRONTEND_LOADBALANCER: kube-frontend-lb

# Backend Services
# Current: kube-backend, kube-backend-lb
# Change to: your-backend-service-names
BACKEND_SERVICE: kube-backend
BACKEND_LOADBALANCER: kube-backend-lb

# Database Service
# Current: postgres
# Change to: your-database-service-name
POSTGRES_SERVICE: postgres
```

### ConfigMap and Secret Names
```yaml
# ConfigMaps
# Current: backend-config, postgres-config, postgres-init
# Change to: your-configmap-names
BACKEND_CONFIGMAP: backend-config
POSTGRES_CONFIGMAP: postgres-config
POSTGRES_INIT_CONFIGMAP: postgres-init

# Secrets
# Current: backend-secret, postgres-secret
# Change to: your-secret-names
BACKEND_SECRET: backend-secret
POSTGRES_SECRET: postgres-secret
```

### Application Labels
```yaml
# App Labels
# Current: app: kube-frontend, app: kube-backend, app: postgres
# Change to: your-app-labels
FRONTEND_LABEL: app: kube-frontend
BACKEND_LABEL: app: kube-backend
POSTGRES_LABEL: app: postgres
```

Remember to update all references to these values throughout your manifests when customizing! 