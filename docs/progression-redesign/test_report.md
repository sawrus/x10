# Progression Redesign Test Report

## Automated Checks

- `make test`: passed
- `make lint`: passed
- `cargo test`: passed
- `web/test.sh`: passed

## Covered Scenarios

### Backend

- default level seeding on profile creation
- signed balance generation from task execution
- level lookup from balance thresholds
- photo upload summary generation
- protected route rejection on actor/profile mismatch
- docs and web routes served successfully

### Frontend Smoke

- web bundle exists in `web/dist`
- core files `index.html`, `app.css`, and `app.js` are produced
- the built app includes dashboard and theme-switch markers

## Manual Review Notes

- web UI supports profile bootstrap, task planning, task execution, photo upload, theme switching, and day finalization
- balance chart updates from `profile_balances`
- `request_id` is now present in the error envelope and response header

## Residual Risks

- deleting historical executions rebuilds ledger rows; deeper regression coverage would be useful for long histories
- photo image dimensions are not extracted yet and remain nullable metadata
- frontend testing is smoke-level and not full browser E2E
