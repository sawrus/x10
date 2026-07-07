# Epic 00 Plan

## Increments

1. `00-06 Query provider`
   - Add shared React Query provider defaults.
   - Move health integration into a reusable query hook.

2. `00-07 Zustand app store`
   - Add app-level client store for `profileId`, UI flags, and game settings.
   - Keep server state out of Zustand.

3. `00-08 UI primitives`
   - Create typed `shared/ui` primitives for layout, input, feedback, and modal patterns.
   - Reuse primitives in the current skeleton pages.

4. `00-09 Event bus`
   - Define typed React-to-Phaser event contract in `shared/lib`.
   - Keep the implementation Phaser-agnostic.

5. `00-10 Build and release checks`
   - Add `lint`, `typecheck`, `build`, and `check` scripts for `web/game`.
   - Update docs, progress, changelog, and version artifacts.

## Dependency Order

- `00-06` precedes future domain query hooks.
- `00-07` feeds local profile context and UI state.
- `00-08` stabilizes page composition after state foundations exist.
- `00-09` lands on top of shared foundations with no backend changes.
- `00-10` closes the epic after integration and verification.
