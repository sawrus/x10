---
trigger: model_decision
glob: logging-observability-guide
description: structured logging, metrics, and error context
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/logging-observability-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Logging & Observability Rule

**Rules:**

- Use structured logs (JSON or similar) with timestamps, context, Task IDs, user IDs.
- Avoid logging secrets or PII.
- Log errors with stack trace and actionable info.
- Use loguru for Python projects (from loguru import logger)
- Emit metrics for key events and performance.

**Violations:**

- Logs are free text only.
- Used print or default python logger in code
- Sensitive info is logged.
- Metrics or errors lack context.
