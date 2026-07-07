---
trigger: model_decision
glob: api-design-guide
description: API contracts must be explicit, validated, and versioned
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/api-design-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# API Design Rule

**Rules:**

- Use Pydantic for request/response validation; enforce strict types, no raw dicts.
- Implement `/liveness` and `/readiness` for K8s probes.
- Implement `/metrics` for FastAPI using PrometheusMiddleware.
- Version APIs in path or headers (v1, v2…).
- Sanitize input; output must be explicit.
- Document all endpoints with OpenAPI/Swagger.
- No side effects in GET requests.
- Responses must include status codes and error details.

**Violations:**

- API schemas are implicit.
- Validation is ad-hoc.
- Endpoints mutate state unexpectedly.
