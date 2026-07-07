---
name: api-design-principles
type: skill
description: REST API design decisions — URL conventions, error contracts, versioning, pagination, idempotency, auth patterns.
related-rules:
  - api-design-guide.md
  - security-guide.md
allowed-tools: Read, Write, Edit, Bash
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/skills/api-design-principles/SKILL.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# API Design Principles Skill

> Practical reference for consistent, production-ready API design decisions.

## URL & Method Conventions

```
✅ Plural nouns, kebab-case, resource hierarchy max 2 levels
   GET    /users/{id}
   POST   /orders
   PATCH  /orders/{id}
   DELETE /orders/{id}
   POST   /orders/{id}/cancel      ← actions as sub-resource verbs

❌ Verbs in base path
   POST /createOrder
   GET  /getUser?id=123
```

| Operation | Method | Success code |
|---|---|---|
| Create | POST | 201 |
| Read | GET | 200 |
| Full update | PUT | 200 |
| Partial update | PATCH | 200 |
| Delete | DELETE | 204 |
| Async action | POST | 202 |

## Standard Error Contract

Every error must follow the same shape — never return raw exception messages.

```json
{
  "error": {
    "code": "ORDER_NOT_FOUND",
    "message": "Order ord_123 not found",
    "details": [{ "field": "items[0].quantity", "issue": "must be > 0" }],
    "request_id": "req_abc123"
  }
}
```

```python
# FastAPI
raise HTTPException(
    status_code=404,
    detail={"code": "ORDER_NOT_FOUND", "message": f"Order {id} not found",
            "request_id": request.state.request_id}
)
```

## Pagination

**Cursor-based** — preferred for live/large datasets:
```python
class PaginatedResponse(BaseModel, Generic[T]):
    items: List[T]
    next_cursor: Optional[str] = None   # base64-encoded, opaque to client

def encode_cursor(last_id: int) -> str:
    return base64.b64encode(str(last_id).encode()).decode()
```

**Offset-based** — only for small static datasets where total count is cheap.

## Versioning

```
URL versioning for breaking changes:   /api/v1/orders → /api/v2/orders
Header for minor variations:           Accept: application/vnd.myapi.v2+json

Rules:
- v1 stays alive ≥ 6 months after v2 launch
- Deprecated: return  Deprecation: true  +  Sunset: <date>  headers
- Never remove a field without a major version bump
```

## Idempotency (POST mutations)

```python
@router.post("/orders", status_code=201)
async def create_order(
    body: CreateOrderRequest,
    idempotency_key: Optional[str] = Header(None, alias="X-Idempotency-Key"),
):
    if idempotency_key:
        cached = await redis.get(f"idempotency:{idempotency_key}")
        if cached:
            return JSONResponse(json.loads(cached), status_code=200)

    order = await order_service.create(body)

    if idempotency_key:
        await redis.setex(f"idempotency:{idempotency_key}", 86400, order.model_dump_json())
    return order
```

## Security Checklist

- [ ] Auth middleware applied before handler — never inside handler
- [ ] Ownership check: `if resource.owner_id != current_user.id: raise 403`
- [ ] All inputs validated via Pydantic/Zod before use
- [ ] Rate limiting on public + auth endpoints
- [ ] No secrets or PII in URL parameters
- [ ] CORS: no `allow_origins=["*"]` in production
- [ ] Response never leaks internal IDs, stack traces, or system paths

## Response Design Checklist

- [ ] Consistent envelope: `{ data: T }` or flat — choose one, never mix
- [ ] Dates always ISO 8601 with timezone: `2024-03-15T14:30:00Z`
- [ ] Monetary values: string decimal or integer cents — never float
- [ ] Nullable fields explicit in schema (not absent)
- [ ] List responses always return array, never `null` when empty
