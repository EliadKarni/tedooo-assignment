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
- The Android app is written in Kotlin for modern Android development.
- Docker is used for containerization and orchestration.