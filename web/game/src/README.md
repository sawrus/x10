# Frontend Layers

`src/` follows a shallow Feature-Sliced structure so the game frontend can grow without mixing routing, page composition, and domain state.

## Layers

- `app` — application bootstrap, providers, router, and other top-level wiring.
- `pages` — route-level screens assembled from widgets, features, and entities.
- `widgets` — larger page sections composed from several lower-level slices.
- `features` — user actions and interaction-focused UI, such as toggles and forms.
- `entities` — domain models and related state/helpers for concrete business concepts.
- `shared` — cross-cutting configuration, styles, utilities, and UI primitives with no business knowledge.

## Import Direction

- Move dependencies downward: `app -> pages -> widgets -> features -> entities -> shared`.
- Use `shared` only for generic pieces.
- Keep page files thin and prefer exporting slice public APIs through local `index.ts` files when that reduces import noise.

## Current Entrypoints

- `main.tsx` bootstraps React and mounts `AppProviders`.
- `app/router.tsx` defines the `/game` router basename and attaches route pages.
- `pages/dashboard` contains the current game skeleton screen.
