# Progression Redesign Epic Plan

## Goal

Transform `x10` from a snapshot-based tracker into an event-driven progression game with:

- planned task templates
- repeatable task executions
- append-only balance history
- per-profile levels
- local photo uploads
- game-style web UI

## Increments

### 1. Architecture Runway

- freeze the `v2` schema
- add versioned migrations
- document balance, level, and ownership rules

Acceptance:

- new schema is captured in docs and code
- backend no longer depends on `daily_snapshots`

### 2. Backend Redesign

- implement new repository, domain, service, and API model
- add photo upload, levels, executions, balances, and day finalizations

Acceptance:

- `make lint`
- `make test`
- `make build`

### 3. Web MVP

- add `web/` app with build and smoke test scripts
- serve the bundle from `/app/`
- support profile bootstrap, task planning, task execution, photo upload, day finalization, chart, and theme switching

Acceptance:

- `/app/` loads from the backend
- `dendy` and `apple` themes both work

### 4. Docs and Hardening

- refresh `README.md`
- add schema catalog and Mermaid flow
- capture test and delivery artifacts

Acceptance:

- feature docs exist under `docs/progression-redesign/`
- API table matches `v2`

## Risks

- deleting an execution requires recalculating later balance rows
- per-profile levels need at least one surviving level row
- local file uploads are suitable for MVP but not for multi-host production

## Deferred

- cloud/object storage
- auth beyond `X-Actor-Id`
- notifications and reminders
- mobile-specific UI optimization beyond responsive stacking
