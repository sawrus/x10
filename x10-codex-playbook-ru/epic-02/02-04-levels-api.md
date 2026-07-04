# Levels API hooks

## Эпик

Epic 02

## Фича

Баланс и уровни

## Контекст

Уровни нужны для RPG прогресса.

## Задача для Codex

Создай hooks для списка уровней профиля. CRUD уровней не выводить в игровой UI, но API типы добавить.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/levels`
- `POST /api/v2/profiles/{profile_id}/levels`
- `PATCH /api/v2/levels/{level_id}`
- `DELETE /api/v2/levels/{level_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/level`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Levels API готов

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
