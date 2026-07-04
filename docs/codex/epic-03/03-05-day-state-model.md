# Day State Model

## Эпик

Epic 03

## Фича

Состояние игрового дня

## Контекст

UI должен явно понимать active/closed/missed day.

## Задача для Codex

Создай day state helper: active, closed, missed. Используй profile timezone и список finalizations. Покажи статус дня на dashboard.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/days/finalizations`


## Ожидаемые файлы/зоны изменений

- `src/entities/day/lib`
- `src/pages/dashboard`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Статус дня отображается

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
