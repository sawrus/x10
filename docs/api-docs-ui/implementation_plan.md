# Implementation Plan

## Architecture

- keep business logic unchanged in `application` and `domain`
- add OpenAPI generation and Scalar UI wiring in `src/api/`
- annotate existing HTTP payloads and serialized models with `utoipa` schemas

## Files

- `Cargo.toml`
- `Makefile`
- `README.md`
- `CHANGELOG.md`
- `src/api/mod.rs`
- `src/api/openapi.rs`
- `src/api/routes.rs`
- `src/api/error.rs`
- `src/domain/models.rs`
- `src/domain/scoring.rs`

## Risks

- OpenAPI schemas must stay aligned with the serialized HTTP contract
- same-origin serving avoids CORS work, so the docs path should remain on the backend service
- protected routes still depend on manual `X-Actor-Id`, so helper docs need to stay explicit
