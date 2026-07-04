# Spheres API hooks

## Эпик

Epic 01

## Фича

Сферы жизни

## Контекст

Сферы жизни — характеристики персонажа.

## Задача для Codex

Создай типы и hooks для списка, создания, обновления и удаления сфер.

## Используемое API

- `GET /api/v2/spheres`
- `POST /api/v2/spheres`
- `GET /api/v2/spheres/{sphere_id}`
- `PATCH /api/v2/spheres/{sphere_id}`
- `DELETE /api/v2/spheres/{sphere_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/sphere`
- `src/features/spheres`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Spheres API готов
- [ ] Ошибки отображаются

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
