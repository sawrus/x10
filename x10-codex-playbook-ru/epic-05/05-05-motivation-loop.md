# Motivation loop

## Эпик

Epic 05

## Фича

Цикл мотивации

## Контекст

Нужны причины возвращаться.

## Задача для Codex

Добавь DailyReward/WeeklyReview placeholders на основе finalizations/executions. Без сложного backend, только frontend view по имеющимся данным.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/executions`
- `GET /api/v2/profiles/{profile_id}/days/finalizations`


## Ожидаемые файлы/зоны изменений

- `src/features/motivation`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Показывается daily/weekly мотивация

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
