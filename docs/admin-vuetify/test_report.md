# Test Report

## Automated checks

Executed on July 3, 2026:

- `make fmt` ✅
- `make lint` ✅
- `make test` ✅

## Covered scenarios

### Auth/session

- login endpoint wired with Argon2 hash verification
- protected `/api/v2/*` access requires a valid admin cookie
- logout clears the cookie
- session inspection endpoint returns current auth state

### Backend regression

- protected profile read still rejects mismatched `X-Actor-Id`
- docs UI and root web routes are available
- execution creation still updates the dashboard balance correctly
- photo upload still returns the expected summary URL
- SQLite repository tests still cover level/bootstrap/balance behavior

### Frontend smoke

- Vite build succeeds
- output contains root `index.html`
- output contains `/game` placeholder
- built assets include hashed JS and CSS bundles

## Manual QA notes

- Theme and language controls are implemented in the app shell and are persisted in local storage.
- Destructive actions in the admin UI go through a confirmation dialog.
- Balances and level state are presented as read-only views.
- Walkthrough GIF captured at [admin-walkthrough.gif](/home/lab/work/sawrus/x10/docs/admin-vuetify/admin-walkthrough.gif).

## Residual risks

- A full browser-driven CRUD walkthrough was not automated in this report.
- GIF capture remains a manual/visual deliverable and should be refreshed after any UI polish changes.
