---
applyTo: "android/**"
---
## Scope
These instructions apply only to the Android client code under /android. :contentReference[oaicite:4]{index=4}

## Android requirements (must implement)
Screens:
1) Feed screen
- Scrollable product list
- Pull-to-refresh
- Infinite scroll with loading indicator
2) Product detail screen
- Shows product + seller info

Technical:
- Use Retrofit or Ktor for networking
- UI states: loading, error, empty, content
- Cache product list in memory and survive configuration changes

## Implementation guidance
- Prefer MVVM:
  - ViewModel holds the in-memory cache (items + nextCursor) so it survives rotation.
  - Expose state via StateFlow/LiveData and render based on UiState.
- Networking:
  - Base URL should be configurable (emulator usually uses http://10.0.2.2:8080).
- Pagination:
  - Use next_cursor from backend; do not implement offset paging.
  - Prevent duplicate concurrent “load next page” calls.
- UX:
  - Provide retry actions for initial load and pagination failures.
  - Keep main thread non-blocking; do IO in coroutines/dispatchers.

## Keep API contract stable
- Match backend JSON shapes and fields exactly.
- Handle error responses gracefully.