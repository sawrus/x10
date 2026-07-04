# Profile API hooks

## Эпик

Epic 01

## Фича

Профиль персонажа

## Контекст

Нужно подключить профиль как персонажа игрока.

## Задача для Codex

Создай типы DTO и hooks для create/get/update profile через TanStack Query.

## Используемое API

- `POST /api/v2/profiles`
- `GET /api/v2/profiles/{profile_id}`
- `PATCH /api/v2/profiles/{profile_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/profile`
- `src/features/profile`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Profile API hooks готовы
- [ ] Учитывается X-Actor-Id

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
