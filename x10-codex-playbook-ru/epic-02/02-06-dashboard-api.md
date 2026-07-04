# Dashboard API hook

## Эпик

Epic 02

## Фича

Главный экран

## Контекст

Backend уже отдаёт dashboard агрегат.

## Задача для Codex

Создай useDashboard(profileId) hook. Типизируй Dashboard DTO по реальному ответу, если типы доступны; иначе сделай безопасный минимальный тип и TODO в docs.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/dashboard`


## Ожидаемые файлы/зоны изменений

- `src/entities/dashboard`
- `src/pages/dashboard`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Dashboard hook работает
- [ ] Есть loading/error

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
