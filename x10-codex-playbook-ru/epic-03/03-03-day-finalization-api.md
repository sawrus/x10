# Day Finalization API

## Эпик

Epic 03

## Фича

Игровой день

## Контекст

Backend поддерживает закрытие дня.

## Задача для Codex

Создай hooks для finalize day, list finalizations, delete finalization.

## Используемое API

- `POST /api/v2/profiles/{profile_id}/days/{date}/finalize`
- `GET /api/v2/profiles/{profile_id}/days/finalizations`
- `DELETE /api/v2/day-finalizations/{finalization_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/day`
- `src/features/days`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Day finalization API готов

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
