# Admin Vuetify Frontend

## Summary

This feature replaces the previous game-style `/app/` frontend with a Vuetify CRUD admin panel at `/`. The panel authenticates through backend-managed admin credentials and lets operators manage the SQLite data model without manual API calls.

## User-facing behavior

- `/` opens the admin login screen and, after authentication, the Vuetify CRUD workspace.
- `/game` opens a separate static placeholder for the future game client.
- The admin panel supports light and dark themes.
- The admin panel supports English and Russian.
- Theme and language preferences persist in local storage.

## CRUD scope

- Full CRUD:
  - profiles
  - profile photos
  - spheres
  - tasks
  - executions
  - levels
  - day finalizations
- Read-only:
  - balances
  - profile level state

## Auth/session behavior

- Login is handled by `POST /api/admin/auth/login`.
- Credentials are validated against backend ENV.
- Successful login sets an `x10_admin_session` HTTP-only cookie.
- Logout is handled by `POST /api/admin/auth/logout`.
- All `/api/v2/*` and non-auth `/api/admin/*` routes require a valid admin session.
- Profile-scoped `v2` routes still require `X-Actor-Id`; the admin UI uses the currently selected profile id.

## GIF defaults

- Primary walkthrough language: `en`
- Primary walkthrough theme: `light`
- Captured walkthrough: [admin-walkthrough.gif](/home/lab/work/sawrus/x10/docs/admin-vuetify/admin-walkthrough.gif)

## Non-goals

- `/game` is not a real game client yet.
- No RBAC or multi-admin role model is introduced.
- Balances and profile level state are not directly editable.
