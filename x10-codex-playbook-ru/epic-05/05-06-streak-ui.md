# Streak UI

## Эпик

Epic 05

## Фича

Цикл мотивации

## Контекст

Streak — простой мотиватор.

## Задача для Codex

Рассчитай streak по day_finalizations и покажи на Dashboard. Учитывай timezone профиля.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/days/finalizations`
- `GET /api/v2/profiles/{profile_id}`


## Ожидаемые файлы/зоны изменений

- `src/entities/day/lib`
- `src/widgets`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Streak отображается

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
