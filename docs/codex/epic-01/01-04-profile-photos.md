# Аватар персонажа

## Эпик

Epic 01

## Фича

Профиль персонажа

## Контекст

Backend поддерживает загрузку и выбор фото профиля.

## Задача для Codex

Добавь API hooks и UI для загрузки, списка, выбора и удаления фото профиля.

## Используемое API

- `POST /api/v2/profiles/{profile_id}/photos`
- `GET /api/v2/profiles/{profile_id}/photos`
- `GET /api/v2/photos/{photo_id}`
- `DELETE /api/v2/photos/{photo_id}`
- `POST /api/v2/profiles/{profile_id}/photos/{photo_id}/select`


## Ожидаемые файлы/зоны изменений

- `src/features/profile-photo`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Можно загрузить фото
- [ ] Можно выбрать активное фото

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
