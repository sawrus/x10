# Analytics charts

## Эпик

Epic 05

## Фича

История и аналитика

## Контекст

Игроку нужны графики прогресса.

## Задача для Codex

Добавь простые графики: balance over time, executions by sphere, positive vs negative. Можно использовать лёгкую chart library или SVG/HTML без зависимости.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/balances`
- `GET /api/v2/profiles/{profile_id}/executions`


## Ожидаемые файлы/зоны изменений

- `src/pages/history`
- `src/widgets/analytics`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Графики отображаются

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
