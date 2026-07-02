# Architecture Notes

## Runtime Shape

- `src/api/`: `v2` REST surface, OpenAPI, request ids, web bundle serving
- `src/application/`: validation, ownership checks, file storage, orchestration
- `src/domain/`: event-driven types plus balance and level helpers
- `src/infrastructure/`: SQLite persistence and versioned migration boot
- `web/`: standalone frontend bundle copied from `web/src` to `web/dist`

## Persistence Model

- `profiles`: identity metadata and optional `current_photo_id`
- `profile_photos`: metadata for locally uploaded images
- `spheres`: lightweight categories with weight
- `tasks`: planned templates with `planned_weight`, `planned_score`, `planned_rate`, cadence, and `starts_on`
- `task_executions`: actual completion events
- `profile_balances`: append-only ledger rows derived from executions
- `levels`: per-profile level ladder keyed by `min_balance`
- `profile_level_state`: current level pointer for each profile
- `day_finalizations`: ritual day close markers

## Progression Rules

- each execution inserts one balance row
- positive task => `actual_weight = +planned_weight`
- negative task => `actual_weight = -planned_weight`
- `balance_after` is cumulative from earlier ledger rows
- current level is resolved from the latest balance against the profile's level table

## Ownership Rules

- the acting profile id from `X-Actor-Id` must match all profile-owned resources
- spheres are shared reference data
- photos cannot be deleted while selected as `current_photo_id`

## Migration Policy

- `schema_migrations` tracks applied versioned SQL files
- the codebase intentionally follows a breaking `v2` path
- old `daily_snapshots` behavior is removed from runtime, not dual-written

## Web Delivery

- backend serves `web/dist` at `/app/`
- `web/build.sh` copies source assets into `web/dist`
- `web/test.sh` provides a smoke-level verification layer
