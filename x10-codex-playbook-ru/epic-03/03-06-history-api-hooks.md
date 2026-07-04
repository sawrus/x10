# History API composition

## Эпик

Epic 03

## Фича

История прогресса

## Контекст

История собирается из executions, balances, tasks, spheres.

## Задача для Codex

Создай composition hook useProgressHistory(profileId), который объединяет executions, balances, tasks, spheres в удобную ViewModel.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/executions`
- `GET /api/v2/profiles/{profile_id}/balances`
- `GET /api/v2/profiles/{profile_id}/tasks`
- `GET /api/v2/spheres`


## Ожидаемые файлы/зоны изменений

- `src/features/history`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] History VM готова

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
