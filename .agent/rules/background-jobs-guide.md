---
trigger: model_decision
glob: background-jobs-guide
description: manage background tasks safely and reliably
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/background-jobs-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Background Jobs Rule

**Rules:**

- Use queues for async jobs (Celery, RQ, etc.).
- Ensure idempotency for retried jobs.
- Log errors and metrics for each task.
- Limit concurrency and handle exceptions explicitly.

**Violations:**

- Jobs are non-idempotent.
- Exceptions crash the worker.
- No logging/metrics for task execution.
