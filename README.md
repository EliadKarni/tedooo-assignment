# tedooo-assignment

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
- The Android app is written in Kotlin.
- Docker is used for containerization and orchestration.

## Backend Environment Setup

### Running Backend on Host (with Cargo)

To run the backend directly on your host device using Cargo, you must create an `.env` file in the `backend/` directory:

```
backend/.env
```

With the following environment variables:

```
LOG_LEVEL=info
DB_HOST=your_database_host
DB_PORT=your_database_port
DB_NAME=your_database_name
DB_USER=your_database_user
DB_PASS=your_database_password
CACHE_HOST=your_redis_host
CACHE_PORT=your_redis_port
```

All variables are required for the backend to start successfully.

---

### Running Backend with Docker Compose

To run the backend using Docker Compose, you must create a `secrets/` directory at the project root with the following files:

```
secrets/
   tedooo_db_password.txt
   tedooo_db_root_password.txt
   tedooo_db_user.txt
```

Each file should contain the corresponding value (e.g., `tedooo_db_password.txt` contains the database password, etc.).

These secrets are mounted into the containers by Docker Compose for secure configuration.

---

## Architecture Decisions & Tradeoffs (Backend)

### 1. Web Framework: Axum
- **Decision**: Chosen **Axum** as the web framework.
- **Reason**: It provides best-in-class performance, strong type safety, and seamless integration with the **Tokio** asynchronous runtime, ensuring the backend can handle high concurrency efficiently.

### 2. Security: Docker Secrets
- **Decision**: Sensitive credentials (DB passwords) are managed via **Docker Secrets**.
- **Reason**: This prevents secrets from being exposed in environment variables or source control, adhering to security best practices for containerized applications.

### 3. Code Organization: Separated SQL Queries
- **Decision**: SQL queries are separated into dedicated files/modules.
- **Reason**: This keeps the codebase clean and readable, separating raw SQL logic from the application's business logic and control flow.

### 4. Data Transfer Objects (DTOs)
- **Decision**: Strict separation between Database Entities (Rows) and API Models (DTOs).
- **Reason**: Decouples the internal database schema from the external API contract, allowing independent evolution of both and preventing accidental exposure of internal fields.

### 5. Test Data Generation
- **Decision**: Included `generate_products` and `generate_sellers` endpoints.
- **Reason**: Facilitates easy population of the database for testing and development purposes without needing external scripts.

### 6. Caching: Lazy Implementation (Cache-Aside)
- **Decision**: Implemented a **Lazy (Cache-Aside)** caching strategy.
- **Reason**: Data is loaded into Redis only when requested. This ensures efficient memory usage (only active data is cached) and resilience (system works even if cache is empty/down), while maintaining eventual consistency.

## What would you add for production

### 1. Authentication & Authorization
- Implement **JWT (JSON Web Token)** based authentication to secure the API.
- Add a **Login Screen** to the Android application to handle user sessions.

### 2. Security Enhancements
- **SQL Injection**: While `sqlx` currently handles parameter binding, I would enforce strict linting and code reviews to ensure no raw SQL string concatenation is ever used.
- **XXE Protection**: If XML support is added for bulk product/seller uploads, I would implement strict XML parsing configurations to prevent **XML External Entity (XXE)** attacks.
- **Reverse Proxy**: Add a reverse-proxy container to docker-compose to terminate TLS, manage certificates, and enforce rate limiting for basic DDoS mitigation.

### 3. User Experience (UX)
- **Search**: Add a search bar to the Android feed screen to search by seller (name or id) and product (title or id).
- **Animations**: Add smooth transitions and animations to the Android app for a more polished feel.
- **Informative Display**: Improve error handling and loading states in the UI to give users better feedback (e.g., skeleton screens, specific error messages instead of generic toasts).



