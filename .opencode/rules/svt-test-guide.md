---
trigger: always_on
glob: svt-test-guide
description: enforce simultaneous user/system tests on simplified data
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/svt-test-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Rule — SVT Test

**Purpose:** Verify system stability under concurrent usage.

- Run N users/systems on simple data.
- Simulate load (e.g., Locust for FastAPI).
- Check outputs and service logs.
- Run via Makefile: `make svt-test`.
- Must **not** be confused with unit tests.

**Violations:** Missing SVT test, logs contain errors, concurrency failures.
