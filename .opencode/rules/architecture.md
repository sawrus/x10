---
agentic:
  generated_by: agentic
  source: "areas/software/frontend/rules/architecture.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.5.1"
  updated_by: "v0.6.0"
---
# Rule: Component Architecture

**Priority**: P0 — Structural violations require refactor before shipping.

## Constraints

1. **Atomic Design Boundary**: Atoms have no internal state. Molecules compose atoms. Organisms own local UI state. Pages own data-fetching. Templates are layout-only.
2. **No Prop Drilling beyond 2 levels**: Pass data via context, state management, or composition — never chain props more than 2 components deep.
3. **Co-location**: A component's styles, tests, and stories must live in the same directory as the component file.
4. **Single Responsibility**: One component = one visual responsibility. A component that fetches data AND renders a complex layout must be split into a container and a presentational component.
5. **No Business Logic in Components**: Domain logic must live in custom hooks (`use*.ts`) or utility modules (`lib/`), not in JSX.
6. **No Circular Imports**: Module dependency graph must be a DAG.

## Directory Convention

```
src/
├── components/        # Atoms, Molecules, Organisms (pure UI)
├── features/          # Feature slices (container + domain logic)
│   └── [feature]/
│       ├── components/
│       ├── hooks/
│       ├── store/
│       └── types.ts
├── pages/             # Route-level components (data orchestration)
├── lib/               # Shared utilities, helpers, constants
├── hooks/             # Shared custom hooks
└── types/             # Global TypeScript types
```
