# Complete Quest Dialog

## Эпик

Epic 02

## Фича

Выполнение квестов

## Контекст

Игрок должен оценивать качество выполнения задачи.

## Задача для Codex

Создай модальное окно выполнения квеста: actual_score 1-5, actual_rate 0-100, completed_at optional. После submit создать execution.

## Используемое API

- `POST /api/v2/tasks/{task_id}/executions`


## Ожидаемые файлы/зоны изменений

- `src/features/executions/complete-quest`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Execution создаётся
- [ ] Список квестов/дашборд инвалидируются

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
