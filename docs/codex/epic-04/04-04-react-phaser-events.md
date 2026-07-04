# React ↔ Phaser events

## Эпик

Epic 04

## Фича

Game Feedback Layer

## Контекст

Игровой слой должен реагировать на события frontend.

## Задача для Codex

Подключи GameEventBus к Phaser. Реализуй обработчики quest:completed, balance:changed, level:up, sphere:highlight.


## Ожидаемые файлы/зоны изменений

- `src/widgets/game-map`
- `src/shared/lib/game-event-bus.ts`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] События доходят до Phaser

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
