# Объяснение экономики

## Эпик

Epic 02

## Фича

Правила игровой экономики

## Контекст

Игрок должен понимать, почему получил очки или штраф.

## Задача для Codex

Добавь компонент BalanceChangeExplanation. Показывай planned_weight, planned_score/rate, actual_score/rate и итоговое изменение, если эти данные доступны из execution/balance/task.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/balances`
- `GET /api/v2/profiles/{profile_id}/executions`


## Ожидаемые файлы/зоны изменений

- `src/features/balances`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Изменение баланса объяснимо

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
