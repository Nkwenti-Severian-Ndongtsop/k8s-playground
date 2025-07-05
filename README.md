# Kubernetes Playground - Full Stack Application

A complete full-stack web application deployed on Kubernetes with frontend, backend, and PostgreSQL database for playing around with k8s internal service communication.

## 🏗️ Architecture

- **Frontend**: HTML/CSS/JavaScript with user authentication interface
- **Backend**: Rust/Axum REST API with JWT authentication
- **Database**: PostgreSQL for user data persistence
- **Orchestration**: Kubernetes with LoadBalancer services

## 🚀 Quick Start

### Prerequisites

- Kubernetes cluster (minikube, kind, k3s, or cloud provider)
- kubectl configured
- Docker for building images
- Docker registry access (or use local images)


## 📁 Project Structure

```
k8s-playground/
├── backend/                 # Rust backend application
│   ├── src/main.rs         # Main backend code
│   ├── Cargo.toml          # Rust dependencies
│   └── Dockerfile          # Backend container
├── frontend/               # Frontend application
│   ├── index.html          # Main page
│   ├── login.html          # Login page
│   ├── register.html       # Registration page
│   ├── profile.html        # Profile page
│   ├── script.js           # Frontend JavaScript
│   ├── style.css           # Styling
│   └── Dockerfile          # Frontend container
├── db/                     # Database initialization
│   └── init.sql           # PostgreSQL schema
├── k8s-manifests/         # Kubernetes manifests
│   ├── postgres-*.yaml    # Database configuration
│   ├── backend-*.yaml     # Backend configuration
│   └── frontend-*.yaml    # Frontend configuration
├── explanation.md          # Detailed architecture explanation
└── README.md              # This file
```

## 🔐 Security Notes

- Database credentials are stored in Kubernetes Secrets
- JWT tokens expire after 24 hours
- Passwords are hashed using bcrypt
- CORS is configured to allow cross-origin requests
- Database is only accessible within the cluster

## 📚 Additional Resources

- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Rust Axum Framework](https://github.com/tokio-rs/axum)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is open source and available under the MIT License. 