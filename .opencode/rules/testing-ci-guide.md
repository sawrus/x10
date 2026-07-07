---
trigger: always_on
glob: testing-ci-guide
description: enforce unit, integration, and e2e testing with formatting and deployment checks
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/testing-ci-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Testing & CI Rule

**Rules:**

- Every new code file must have a corresponding unit test file.
- Run formatting & linting and fix until passed.
- Run unit tests and ensure coverage meets the required threshold (default ≥70%). Add tests for positive/negative scenarios.
- Start dependent services and ensure logs are clean.
- Apply migrations without errors.
- Develop blackbox e2e-test with input data; full API scenario must pass.

**Violations:**

- Missing unit tests.
- Coverage below threshold.
- Format/lint errors not fixed.
- Service logs contain errors.
- Migrations fail.
- E2E test fails.
