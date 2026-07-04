# Execution API hooks

## Эпик

Epic 02

## Фича

Выполнение квестов

## Контекст

Выполнение квеста — главный игровой action.

## Задача для Codex

Создай hooks для create/list/get/delete task executions.

## Используемое API

- `POST /api/v2/tasks/{task_id}/executions`
- `GET /api/v2/profiles/{profile_id}/executions`
- `GET /api/v2/executions/{execution_id}`
- `DELETE /api/v2/executions/{execution_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/execution`
- `src/features/executions`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Execution API готов

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
