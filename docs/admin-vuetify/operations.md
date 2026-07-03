# Operations Notes

## Required ENV

- `X10_ADMIN_USERNAME`
- `X10_ADMIN_PASSWORD_HASH`
- `X10_ADMIN_SESSION_SECRET`

Optional:

- `X10_ADMIN_SESSION_SECURE`
- `X10_HOST`
- `X10_PORT`
- `X10_DATABASE_PATH`
- `X10_UPLOADS_PATH`
- `X10_WEB_DIST_PATH`

## Password hash generation

```bash
cargo run --bin hash_admin_password -- '<plain password>'
```

Use the printed Argon2 PHC string as `X10_ADMIN_PASSWORD_HASH`.

## Local commands

```bash
make fmt
make lint
make test
make run
```

## Rollout notes

- Root route behavior changed from redirecting to `/app/` to serving the admin SPA.
- Legacy `/app` now redirects to `/game`.
- Admin auth is now required for progression API access.

## Rollback considerations

- Reverting the feature requires restoring the old static frontend and removing admin auth middleware from `/api/v2/*`.
- If rollback is partial, API consumers may be locked out because the new admin session requirement is enforced server-side.

## Known operational risks

- The generated session cookie is stateless and signed, not database-backed.
- Photo cleanup is best-effort during profile deletion.
- The frontend bundle size is larger because Vuetify and Material Design Icons are shipped in the static output.
