# Epic 00 Risk Register

| Risk | Impact | Mitigation | Residual status |
| --- | --- | --- | --- |
| No full ESLint stack in `web/game` | Style issues can slip through | Added lightweight `npm run lint` plus strict typecheck and documented the scope | Medium |
| Incomplete local frontend dependencies | Blocks local verification in some workspaces | Keep `npm ci` in `web/game` as the first verification step and document the expectation | Medium |
| Theme/profile/UI state split across slices | Could blur ownership of client state | Consolidated active game settings into app store and kept theme values reusable through shared model | Low |
| Future Phaser integration coupling to React | Could force rewrites in Epic 04 | Defined a typed event bus contract in `shared/lib` with no Phaser dependency | Low |
