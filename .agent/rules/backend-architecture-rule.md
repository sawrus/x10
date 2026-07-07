---
trigger: always_on
glob: backend-architecture-rule
description: enforce layered architecture and dependency rules for backend services
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/backend-architecture-rule.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Backend Architecture Rule

**Goal:** Maintain clean separation of concerns and testability.

**Layers:**

1.  **API Layer (Entry Points):**
    - Handles HTTP requests, websocket events, or CLI commands.
    - Validates input using Pydantic schemas.
    - Calls Service Layer.
    - **Allowed Imports:** Service Layer, Schemas.
    - **Forbidden Imports:** Repository Layer, direct DB access.

2.  **Service Layer (Business Logic):**
    - Contains pure business logic and orchestration.
    - Transaction boundaries are defined here.
    - **Allowed Imports:** Repository Layer, Schemas, Domain Models.
    - **Forbidden Imports:** API Layer, frameworks (FastAPI/Flask) specific objects.

3.  **Repository Layer (Data Access):**
    - Abstracts database queries and external service calls.
    - Returns Domain Models or Pydantic Schemas.
    - **Allowed Imports:** Domain Models, Database Drivers (SQLAlchemy/AsyncPG).
    - **Forbidden Imports:** API/Service Layers.

4.  **Domain Models / Schemas:**
    - Pure data structures.
    - **Allowed Imports:** None (should be dependency-free).

**Violations:**

- API layer directly accessing database.
- Service layer returning SQLAlchemy models directly (leaking implementation details).
- Circular dependencies between layers.
- Business logic inside API handlers.
