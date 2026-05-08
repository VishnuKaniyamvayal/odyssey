# rust-odyssey

## April 16, 2026 - Axum Setup

- Fixed serde derive, sqlx postgres features, and module imports
- Learned Axum Router state typing: `Router<Arc<AppState>>` before state, `Router<()>` after
- Fixed route syntax `:id` → `{id}` and set up server listener
- Server running and curl endpoints working

