---
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/skills/api-patterns/rate-limiting.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# Rate Limiting Principles

> Protect your API from abuse and overload.

## Why Rate Limit

```
Protect against:
├── Brute force attacks
├── Resource exhaustion
├── Cost overruns (if pay-per-use)
└── Unfair usage
```

## Strategy Selection

| Type | How | When |
|------|-----|------|
| **Token bucket** | Burst allowed, refills over time | Most APIs |
| **Sliding window** | Smooth distribution | Strict limits |
| **Fixed window** | Simple counters per window | Basic needs |

## Response Headers

```
Include in headers:
├── X-RateLimit-Limit (max requests)
├── X-RateLimit-Remaining (requests left)
├── X-RateLimit-Reset (when limit resets)
└── Return 429 when exceeded
```
