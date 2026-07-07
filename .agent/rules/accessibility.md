---
agentic:
  generated_by: agentic
  source: "areas/software/frontend/rules/accessibility.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# Rule: Accessibility First

**Priority**: P0 — Blocks merge if violated.

## Constraints

1. Every interactive element (`button`, `a`, `input`, `select`, custom widget) **must** have an accessible name via `aria-label`, `aria-labelledby`, or visible text content.
2. Color contrast ratio must be **≥ 4.5:1** for normal text and **≥ 3:1** for large text (WCAG 2.1 AA).
3. All functionality must be operable via **keyboard only** (Tab, Shift+Tab, Enter, Space, Arrow keys).
4. Images must have meaningful `alt` attributes; decorative images use `alt=""`.
5. Dynamic content updates (toasts, modals, form errors) must be announced via `aria-live` regions or focus management.
6. No `tabindex` values greater than `0` are permitted.

## Verification

The agent must run `axe-core` or `eslint-plugin-jsx-a11y` checks before considering any UI component complete.
