# Changelog

## 0.3.0

- redesigned the app around versioned `v2` SQLite migrations and removed runtime dependence on `daily_snapshots`
- added profile photos, signed balance ledger rows, per-profile levels, day finalizations, and event-driven task executions
- replaced the old task model with planned task templates using `planned_weight`, `planned_score`, `planned_rate`, `starts_on`, and cadence
- added CRUD-style `v2` API routes for profiles, photos, spheres, tasks, executions, balances, levels, and dashboard reads
- added a game-style web frontend at `/app/` with `dendy` and `apple` themes
- updated README and feature docs for the progression redesign epic

## 0.2.0

- added `utoipa` and `utoipa-scalar` so the backend now serves interactive API docs at `/docs/`
- added `/docs/openapi.json` for the generated OpenAPI document on the same service port
- added `make actor-id` to create a demo profile id for manual `X-Actor-Id` entry in the docs UI
- documented the feature and verification artifacts under `docs/api-docs-ui/`

## 0.1.0

- bootstrapped the first Rust backend for `x10`
- added a layered Axum API with profile, sphere, task, finalization, and progression endpoints
- introduced a SQLite-backed repository with schema bootstrap from `migrations/0001_init.sql`
- added health, metrics, request id logging, tests, and Makefile developer commands
- documented the bootstrap scope, implementation plan, and test report under `docs/backend-bootstrap/`
