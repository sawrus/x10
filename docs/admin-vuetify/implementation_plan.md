# Implementation Plan

## Backend

- Extend config with admin username, password hash, session secret, and secure-cookie toggle.
- Add signed cookie session handling and auth middleware for `/api/v2/*` and protected `/api/admin/*`.
- Add admin-specific endpoints for:
  - login
  - logout
  - session inspection
  - profile listing
  - profile deletion
  - profile level state reads
- Add day-finalization deletion support for the admin workflow.
- Keep progression rules in `application`/`infrastructure`, not in handlers.

## Frontend

- Replace the old static frontend with `Vite + Vue 3 + Vuetify + vue-i18n`.
- Serve the admin SPA from `/`.
- Keep the UI in two main work areas:
  - profile workspace
  - spheres management
- Use shared Vuetify patterns:
  - top app bar
  - navigation drawer
  - dialogs for create/edit
  - confirmation dialog for destructive actions
  - snackbar for status messaging

## Static delivery

- Use `web/dist` as the single build output directory.
- Deliver the game placeholder via `web/public/game/index.html`.
- Redirect legacy `/app` traffic to `/game`.

## Risks addressed

- Admin auth is enforced on the backend, not only in the UI.
- Existing profile ownership logic is preserved by continuing to use `X-Actor-Id` for profile-scoped `v2` routes.
- The frontend build pipeline is updated to handle Vite asset hashing.
