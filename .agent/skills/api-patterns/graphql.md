---
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/skills/api-patterns/graphql.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# GraphQL Principles

> Flexible queries for complex, interconnected data.

## When to Use

```
✅ Good fit:
├── Complex, interconnected data
├── Multiple frontend platforms
├── Clients need flexible queries
├── Evolving data requirements
└── Reducing over-fetching matters

❌ Poor fit:
├── Simple CRUD operations
├── File upload heavy
├── HTTP caching important
└── Team unfamiliar with GraphQL
```

## Schema Design Principles

```
Principles:
├── Think in graphs, not endpoints
├── Design for evolvability (no versions)
├── Use connections for pagination
├── Be specific with types (not generic "data")
└── Handle nullability thoughtfully
```

## Security Considerations

```
Protect against:
├── Query depth attacks → Set max depth
├── Query complexity → Calculate cost
├── Batching abuse → Limit batch size
├── Introspection → Disable in production
```
