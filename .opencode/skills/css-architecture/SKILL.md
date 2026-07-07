---
name: css-architecture
type: skill
description: Structure CSS/Tailwind for maintainability — tokens, BEM naming, specificity control, responsive patterns.
related-rules:
  - architecture.md
  - accessibility.md
allowed-tools: Read, Write, Edit, Bash
agentic:
  generated_by: agentic
  source: "areas/software/frontend/skills/css-architecture/SKILL.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# CSS Architecture Skill

> **Expertise:** Design tokens, BEM, Tailwind utility classes, CSS custom properties, responsive design, dark mode.

## Design Tokens — Never Hardcode Values

```css
/* tokens.css — single source of truth */
:root {
  /* Color — semantic names, not visual */
  --color-primary: #3b82f6;
  --color-primary-hover: #2563eb;
  --color-surface: #ffffff;
  --color-surface-raised: #f8fafc;
  --color-text-primary: #0f172a;
  --color-text-muted: #64748b;
  --color-error: #ef4444;
  --color-success: #22c55e;

  /* Spacing — 4px base unit */
  --space-1: 0.25rem;   /* 4px */
  --space-2: 0.5rem;    /* 8px */
  --space-4: 1rem;      /* 16px */
  --space-6: 1.5rem;    /* 24px */
  --space-8: 2rem;      /* 32px */

  /* Typography */
  --font-size-sm: 0.875rem;
  --font-size-base: 1rem;
  --font-size-lg: 1.125rem;
  --font-size-xl: 1.25rem;
  --line-height-tight: 1.25;
  --line-height-normal: 1.5;

  /* Borders */
  --radius-sm: 0.25rem;
  --radius-md: 0.5rem;
  --radius-lg: 1rem;
  --border-color: #e2e8f0;
}

/* Dark mode via media query or class */
@media (prefers-color-scheme: dark) {
  :root {
    --color-surface: #0f172a;
    --color-text-primary: #f1f5f9;
    --border-color: #1e293b;
  }
}
```

## BEM Naming Convention

```css
/* Block */
.card { }

/* Element (part of block) */
.card__header { }
.card__body { }
.card__footer { }

/* Modifier (variation of block or element) */
.card--featured { }
.card__header--sticky { }

/* Avoid deep nesting — max 2 levels */
/* ❌ .card__header__title__icon { } */
/* ✅ .card__icon { } */
```

## Tailwind: Extracting Repeated Patterns

```tsx
// ❌ Repeating long class strings — hard to maintain
<button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 disabled:opacity-50">
<button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 disabled:opacity-50">

// ✅ Extract to component variant
const buttonVariants = cva(
  "px-4 py-2 rounded-md focus:ring-2 disabled:opacity-50 transition-colors",
  {
    variants: {
      variant: {
        primary: "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500",
        secondary: "bg-gray-100 text-gray-900 hover:bg-gray-200 focus:ring-gray-400",
        ghost: "text-gray-700 hover:bg-gray-100 focus:ring-gray-400",
      },
      size: {
        sm: "px-3 py-1.5 text-sm",
        md: "px-4 py-2",
        lg: "px-6 py-3 text-lg",
      },
    },
    defaultVariants: { variant: "primary", size: "md" },
  }
);
```

## Responsive Design Patterns

```css
/* Mobile-first — base styles for mobile, then override for larger */
.card {
  padding: var(--space-4);           /* mobile */
}

@media (min-width: 768px) {
  .card { padding: var(--space-6); } /* tablet */
}

@media (min-width: 1024px) {
  .card { padding: var(--space-8); } /* desktop */
}

/* Container queries (modern — prefer over media queries for components) */
.card-container { container-type: inline-size; }

@container (min-width: 400px) {
  .card { display: grid; grid-template-columns: auto 1fr; }
}
```

## Specificity Rules

```css
/* Target specificity range: 0-1-0 to 0-2-0 */
/* ❌ Too high — hard to override */
#app .container .card.featured > .header span.title { }  /* 1-3-1 */

/* ✅ Right level — component-scoped */
.card__title { }                 /* 0-1-0 */
.card--featured .card__title { } /* 0-2-0 */

/* Never use !important in component styles — signals architecture problem */
```
