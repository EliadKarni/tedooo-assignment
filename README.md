# todooo-assignment

# Full-Stack Development Assignment

## Project Structure
- `/backend`: Rust-based backend service.
- `/android`: Kotlin-based Android project.
- `/docker-compose.yml`: Orchestrates the backend and Android services.
- `/.github`: Contains CI/CD workflows and Copilot instructions.

## Setup Instructions

### Prerequisites
- Docker and Docker Compose installed.
- Rust and Cargo installed (for local development).
- Android Studio installed (for local development).

### Running the Project
1. Build and run the services:
   ```bash
   docker-compose up --build
   ```
2. Access the backend at `http://localhost:8080`.
3. Access the Android service at `http://localhost:8081`.

## Design Notes
- The backend is built using Rust for performance and safety.
- The Android app is written in Kotlin for modern Android development.
- Docker is used for containerization and orchestration.

## Backend Environment Setup

### Running Backend on Host (with Cargo)

To run the backend directly on your host device using Cargo, you must create an `.env` file in the `backend/` directory with the following environment variables:

```
DB_HOST=your_database_host
DB_PORT=your_database_port
DB_NAME=your_database_name
DB_USER=your_database_user
DB_PASS=your_database_password
RUST_LOG=info
CACHE_HOST=your_redis_host:port
```

**Location:**

```
backend/.env
```

All variables are required for the backend to start successfully.

---

### Running Backend with Docker Compose

To run the backend using Docker Compose, you must create a `secrets/` directory at the project root with the following files:

```
secrets/
   todooo_db_password.txt
   todooo_db_root_password.txt
   todooo_db_user.txt
```

Each file should contain the corresponding value (e.g., `todooo_db_password.txt` contains the database password, etc.).

These secrets are mounted into the containers by Docker Compose for secure configuration.

---

