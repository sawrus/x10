# Epic 00 Architecture Notes

## Runway

- Preserve the shallow Feature-Sliced structure: `app -> pages -> widgets -> features -> entities -> shared`.
- Keep TanStack Query as the only server-state layer.
- Keep Zustand as the only app-level client-state layer for profile context, modal/UI flags, and local game settings.
- Keep the event bus in `shared/lib` so Phaser can attach later without leaking game logic into rendering code.

## Workflow Adaptation

`develop-epic` normally points each increment at `develop-feature`. For this epic we used `develop-feature-fullstack` as requested, but mapped its stages to the real stack:

- design: `docs/codex/epic-00/*.md`
- db/repository/service/api: intentionally no-op for this frontend-only epic
- frontend/ui: `web/game/src/**`
- qa/pr/docs/version: local checks, README, `PROGRESS.md`, `CHANGELOG.md`, and version bump

## Shared Contracts

- `shared/api/health.ts` owns the health request and shared query options.
- `app/store/useAppStore.ts` owns `profileId`, UI flags, and game settings.
- `shared/ui/**` owns reusable presentational components with no business knowledge.
- `shared/lib/game-event-bus.ts` owns typed events for `quest:completed`, `balance:changed`, `level:up`, `day:finalized`, and `sphere:highlight`.
