# Backend Bootstrap

## Problem Statement

`x10` needs a backend-first core that can drive future CLI, TUI, MCP, GUI, mobile, web, and automation clients from one consistent domain model.
The first increment should validate the core loop: profile -> daily tasks -> positive and negative weight accounting -> progression summary.

## First Increment Scope

- single-user backend with a lightweight development auth header
- profile creation and read
- SQLite file persistence for the service state
- sphere listing and creation
- task creation and completion for dated day/week/month/year tasks
- progression engine with:
  - positive and negative weighted tasks
  - cumulative balance score
  - derived level from `x0` to `x10`
  - `rest_day_credits` earned on strong positive days
  - fixed historical daily snapshots for a future graph
- explicit day finalization endpoint
- structured error contract
- health and metrics endpoints

## Non-Goals

- multi-user auth and onboarding
- public profile sharing
- sphere weight multipliers beyond a separate sphere reference
- recurring schedules, reminders, or background jobs
- social features and recommendations

## Domain Assumptions For This Increment

- level is a derived representation of current cumulative balance, not a separate hidden state machine
- each task has a signed effect encoded as `kind` plus positive integer `weight`
- negative actions are modeled as completed negative tasks
- each task belongs to one cadence bucket: `day`, `week`, `month`, or `year`
- if a day reaches the configured positive threshold, the user earns one `rest_day_credit`
- if a day finishes with a negative net score, one `rest_day_credit` is removed if available
- finalized days are stored in `daily_snapshots` and are not re-finalized
- timezone is stored on profile and will matter more once richer scheduling rules are added
- authorization is development-only: `X-Actor-Id` must match the target profile owner on protected routes

## Acceptance Criteria

1. A client can create a profile and receive a stable profile id.
2. A client can read a profile only when `X-Actor-Id` matches the profile id.
3. A client can create dated tasks with `positive` or `negative` kind, positive integer weight, optional `sphere_id`, and cadence `day|week|month|year`.
4. A client can mark a task as completed through a dedicated action endpoint.
5. A client can finalize a day once and receive a stored snapshot for that date.
6. A client can request a progression dashboard and receive:
   - current balance score
   - current level between `x0` and `x10`
   - available `rest_day_credits`
   - current-day totals
   - historical daily snapshots suitable for graphing
7. A client can list available spheres from the separate `spheres` table.
8. All API errors use the common error envelope with a request id.
9. The service exposes `GET /health` and `GET /metrics`.
10. Automated tests cover domain scoring rules and one protected API path.
