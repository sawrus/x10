# Changelog

## 0.4.5

- added `GET /api/v2/profiles` for the game client so the Character screen can reopen existing heroes without relying on admin-only endpoints
- persisted the selected game `profileId` across refreshes and added a saved-hero roster on the Character screen for reopening or switching heroes
- added `created_at` and `updated_at` to public profile responses so the saved-hero roster can show creation or last-edit dates
- fixed the Character roster so selecting another saved hero while a card is already open actually switches the loaded hero
- expanded the Playwright `make e2e-test` scenario to verify refresh recovery and reopening an existing hero from the roster
- expanded the Playwright `make e2e-test` scenario again to verify switching between multiple saved heroes from the roster
- made `make game-stack` inject local fallback admin credentials so the backend no longer crashes when `X10_ADMIN_*` variables are unset during game frontend development
- added `make game-start` as a `game-stack` alias and hardened `make game-stop` so it also frees stale listeners on ports `3000` and `5173`

## 0.4.4

- completed Epic 00 frontend foundations in `web/game` with shared React Query defaults, a typed Zustand app store, reusable UI primitives, and a typed game event bus contract
- added standalone `web/game` `typecheck`, `lint`, `build`, and `check` scripts plus an integrated `game-build` verification flow
- updated README, epic docs, progress tracking, and delivery artifacts for the finished frontend architecture epic

## 0.4.3

- added a shared `web/game` API client with env-driven base URL resolution, JSON request and response handling, typed API errors, and `X-Actor-Id` support
- switched the React dashboard skeleton to query the real `GET /health` endpoint through `src/shared/api` instead of inline placeholder data

## 0.4.2

- wired `/game` to the new React/Vite frontend build instead of the legacy static placeholder
- added `make game-build` and `make game-dev` for the dedicated game client workflow
- updated the web smoke checks and README to reflect the live `/game` client

## 0.4.1

- added the Epic 00 React/Vite/TypeScript game frontend skeleton under `web/game` with TanStack Query, Zustand, React Router, and Tailwind CSS while preserving the existing admin frontend.
- marked frontend stack installation progress in `docs/codex/PROGRESS.md`.

## 0.4.0

- replaced the old `/app/` frontend with a Vuetify CRUD admin panel served from `/`
- added admin authentication via `X10_ADMIN_USERNAME`, `X10_ADMIN_PASSWORD_HASH`, and a signed HTTP-only cookie session
- added `/game` as a separate HTML placeholder route
- added admin profile listing, profile deletion, profile level state reads, and day-finalization deletion support
- switched the frontend build to `Vite + Vue 3 + Vuetify + vue-i18n`
- updated README and shipped feature docs under `docs/admin-vuetify/`

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
