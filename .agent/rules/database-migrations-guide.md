---
trigger: model_decision
glob: database-migrations-guide
description: all DB schema changes must be explicit, versioned, and testable via Alembic
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/database-migrations-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Database & Migration Rule

**Goal:** schema changes must be explicit, versioned, traceable, and tested.

**Rules:**

- Use **Alembic** for all schema changes; never rely only on ORM metadata or manual SQL.
- Each change in a migration script with upgrade/downgrade; idempotent if possible; consistent naming/versioning.
- Include tables, columns, indexes, constraints, relationships explicitly.
- Tests must apply migrations to clean DB and validate structure & integrity (Postgres & SQLite if supported).
- Avoid destructive ops without backup; group related changes; keep scripts small; document dependencies/order.

**Violations:**

- Schema changes outside Alembic.
- Missing tests or incomplete upgrade/downgrade.
- Hardcoded schema in code.
- Drift between environments.
