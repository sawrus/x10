# Test Report

## Executed Checks

- `cargo fmt --all`
- `make test`
- live `HEAD /docs/`
- live `HEAD /docs/openapi.json`
- live `make actor-id`

## Result

- all 7 Rust tests passed
- added HTTP coverage confirms `/docs/` and `/docs/openapi.json` are served by the backend router
- live service verification returned `200 OK` for both docs endpoints on `127.0.0.1:3000`
- `make actor-id` created a profile and printed a usable `X-Actor-Id` value
