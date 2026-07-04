# Task API hooks

## Эпик

Epic 01

## Фича

Доска квестов

## Контекст

Tasks — это квесты игрока.

## Задача для Codex

Создай типы и hooks для создания, списка, получения, обновления и удаления задач.

## Используемое API

- `POST /api/v2/tasks`
- `GET /api/v2/profiles/{profile_id}/tasks`
- `GET /api/v2/tasks/{task_id}`
- `PATCH /api/v2/tasks/{task_id}`
- `DELETE /api/v2/tasks/{task_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/task`
- `src/features/quests`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Task API готов
- [ ] Учитывается profile_id и X-Actor-Id

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
