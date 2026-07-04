# Event Bus React ↔ Phaser

## Эпик

Epic 00

## Фича

Game Feedback Layer

## Контекст

Phaser будет подключён позже, но контракт событий нужен заранее.

## Задача для Codex

Создай простой GameEventBus на TypeScript: `emit`, `on`, `off`. Опиши события `quest:completed`, `balance:changed`, `level:up`, `day:finalized`, `sphere:highlight`.


## Ожидаемые файлы/зоны изменений

- `src/shared/lib/game-event-bus.ts`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Event bus типизирован
- [ ] Есть базовые тесты или пример использования

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
