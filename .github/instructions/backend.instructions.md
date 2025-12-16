---
applyTo: "backend/**"
---

## Scope
These instructions apply only to the Rust backend code under /backend. :contentReference[oaicite:2]{index=2}

## Backend requirements (must implement)
Endpoints:
- GET /products (cursor-based pagination)
- GET /products/{id} (include seller info)
- GET /health (verify DB + Redis connectivity)

Caching:
- List TTL ~30 seconds
- Single product TTL ~5 minutes

## Implementation guidance
- Use async Rust (Axum or Actix are fine).
- Use MySQL via SQLx; migrations should be included and runnable.
- Cursor pagination:
  - Use stable ordering (e.g., created_at + id tie-breaker).
  - Cursor should be treated as opaque by clients (encode/decode internally).
- Cache-aside with Redis:
  - Keys must include pagination inputs for list caching (cursor + limit).
  - If Redis is down, do not fail the request—fallback to DB.
- Database:
  - Provide sensible indexes for pagination and joins.
  - Avoid N+1 queries (join products + sellers where needed).

## API behavior
- Use consistent JSON error format.
- Return proper HTTP codes:
  - 400 invalid cursor/params
  - 404 missing product
  - 503 when DB is unavailable (health should reflect dependency status)