---
agentic:
  generated_by: agentic
  source: "areas/software/frontend/rules/performance.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# Rule: Performance Budget

**Priority**: P1 — Must be resolved before release; tracked per PR.

## Core Web Vitals Targets (Production)

| Metric | Target | Hard Limit |
|:---|:---|:---|
| LCP (Largest Contentful Paint) | < 2.0s | < 2.5s |
| INP (Interaction to Next Paint) | < 100ms | < 200ms |
| CLS (Cumulative Layout Shift) | < 0.05 | < 0.1 |

## Bundle Constraints

1. **Initial JS bundle**: ≤ 200 KB gzipped for the critical path.
2. **Route-level chunks**: Every route must be lazy-loaded via `React.lazy()` + `Suspense`.
3. **Third-party libraries**: Any dependency > 50 KB gzipped must be explicitly approved in `bundle-policy.md`.
4. **No synchronous localStorage access** in the render path.
5. Images must use modern formats (WebP/AVIF) and include `width`/`height` attributes.

## Enforcement

Bundle size is tracked via `bundlesize` in CI. PRs that increase the initial bundle by > 5 KB trigger a mandatory `/bundle-analyze` workflow run.
