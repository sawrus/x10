# Interactive API Docs

## Problem Statement

`x10` needs a browser-based API UI on the same service port so developers can inspect and call the backend without a separate proxy or frontend setup.

## Scope

- serve `utoipa-scalar` from the backend at `/docs/`
- expose the generated OpenAPI document at `/docs/openapi.json`
- keep protected routes callable through manual `X-Actor-Id` entry in the UI
- provide a `make` helper to create a demo profile id for that header

## Non-Goals

- replacing the current development-only auth model
- auto-injecting authorization headers into the docs UI
- hosting documentation on a separate service

## Acceptance Criteria

1. Opening `http://127.0.0.1:3000/docs/` serves a Scalar UI from the same Axum service port.
2. The UI describes the existing backend endpoints and payload schemas.
3. `GET /docs/openapi.json` returns the generated OpenAPI specification.
4. Protected endpoints can be called from the UI by manually setting `X-Actor-Id`.
5. Developers can run `make actor-id` to create a profile id suitable for `X-Actor-Id`.
6. The feature is documented in `README.md`, `CHANGELOG.md`, and this `docs/` artifact set.
