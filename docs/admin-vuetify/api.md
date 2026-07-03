# API Notes

## Auth endpoints

### `POST /api/admin/auth/login`

- Request:
  - `username`
  - `password`
- Success:
  - `200`
  - sets `x10_admin_session` HTTP-only cookie
- Failure:
  - `401 INVALID_CREDENTIALS`
  - `500 INVALID_ADMIN_CONFIG`

### `POST /api/admin/auth/logout`

- Clears the session cookie.

### `GET /api/admin/auth/session`

- Returns:
  - `authenticated`
  - `username`
  - `expires_at`

## Admin endpoints

### `GET /api/admin/profiles`

- Returns every profile for admin selection.

### `DELETE /api/admin/profiles/{profile_id}`

- Deletes the profile and its nested SQLite records.
- Uploaded files are removed from local storage best-effort.

### `GET /api/admin/profiles/{profile_id}/level-state`

- Returns the current `profile_level_state` row for read-only admin inspection.

## Progression endpoints reused by admin UI

The admin UI continues to use the existing `/api/v2/*` contract for profile-scoped work:

- profiles
- photos
- tasks
- executions
- balances
- dashboard
- levels
- day finalizations
- spheres

## Session requirements

- All `/api/v2/*` routes require:
  - a valid admin cookie session
  - `X-Actor-Id` for profile-scoped requests
- All protected `/api/admin/*` routes require:
  - a valid admin cookie session

## Structured errors

All failures follow the existing envelope:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "planned_score must be between 1 and 5",
    "request_id": "..."
  }
}
```
