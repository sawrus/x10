# Implementation Plan

## Architecture

- `domain`: pure entities, value enums, and progression rules
- `application`: use-case orchestration and authorization-aware service methods
- `infrastructure`: repository ports with an in-memory implementation for bootstrap
- `api`: Axum HTTP layer, DTO mapping, request id middleware, metrics, and error contract

Dependency direction:

`api -> application -> domain`

`application -> infrastructure` happens through repository traits only.

## Initial Files

- `src/main.rs`
- `src/lib.rs`
- `src/config.rs`
- `src/domain/`
- `src/application/`
- `src/infrastructure/`
- `src/api/`
- `docs/backend-bootstrap/`
- `Makefile`

## Technical Choices

- Rust 2024
- Axum for REST API
- Tokio runtime
- `tracing` with JSON logs
- Prometheus exporter for lightweight metrics
- SQLite file persistence through a repository trait
- migration bootstrap from `migrations/0001_init.sql`
- separate tables for `profiles`, `spheres`, `tasks`, and `daily_snapshots`

## Risks

- product math is still provisional and may change once the progression formula is finalized
- local auth is intentionally temporary and must be replaced before sharing or multi-user support
- historical snapshots are recomputed from task state today; persistent daily closures should be introduced with database storage

## Follow-Up After This Increment

1. Add update/delete flows and stricter day-finalization rules.
2. Introduce shareable public profile projection and privacy rules.
3. Expand week/month/year scheduling semantics beyond a single anchor date.
4. Add reminders and async jobs only after the base loop is stable.
