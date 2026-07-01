# Delivery Summary

The backend now serves interactive API docs at `/docs/` and the generated OpenAPI document at `/docs/openapi.json` on the same Axum port as the API.

Protected endpoints still rely on manual `X-Actor-Id` entry, and `make actor-id` provides a quick developer flow to create a profile id for that header.

Verification completed with `cargo fmt --all`, `make test`, live checks for both docs endpoints, and a live `make actor-id` run.
