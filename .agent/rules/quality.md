---
agentic:
  generated_by: agentic
  source: "areas/software/frontend/rules/quality.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# Rule: Code Quality

**Priority**: P1 — Violations block merge via automated lint gates.

## Constraints

1. **TypeScript strict mode** is non-negotiable (`strict: true` in `tsconfig.json`). No `any` without an explicit `// eslint-disable` comment explaining why.
2. **No hardcoded user-facing strings**: All copy must use i18n keys (`t('key.path')`).
3. **No direct DOM manipulation**: Never use `document.getElementById` or `querySelector` from within React components. Use `useRef`.
4. **State immutability**: Never mutate state objects directly.
5. **Test coverage**: New components must ship with ≥ 1 rendering test and ≥ 1 interaction test.
6. **No `console.log` in committed code**: Use the project's structured logger (`lib/logger.ts`).
