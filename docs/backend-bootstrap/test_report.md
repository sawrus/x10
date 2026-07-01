# Test Report

## Automated Scenarios

- domain scoring derives `x0..x10` levels from cumulative balance
- positive threshold awards a `rest_day_credit`
- negative daily net removes one `rest_day_credit` when present
- protected profile read rejects mismatched `X-Actor-Id`

## Manual Smoke Checks

- create profile
- create task
- complete task
- fetch dashboard
- inspect `health` and `metrics`

## Open Risks

- persistence durability is not covered in this increment
- daily cutoff rules by timezone are not finalized
- sharing and privacy rules are deferred
