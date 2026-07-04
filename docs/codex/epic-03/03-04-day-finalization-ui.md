# Экран закрытия дня

## Эпик

Epic 03

## Фича

Игровой день

## Контекст

Закрытие дня — ключевой ритуал Life RPG.

## Задача для Codex

Создай DayFinalizationPanel: выполненные задания, баланс дня, note, кнопка 'Завершить день'.

## Используемое API

- `POST /api/v2/profiles/{profile_id}/days/{date}/finalize`
- `GET /api/v2/profiles/{profile_id}/executions`
- `GET /api/v2/profiles/{profile_id}/balances`


## Ожидаемые файлы/зоны изменений

- `src/features/days/finalize`
- `src/widgets/daily-summary`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] День закрывается
- [ ] Повторное закрытие показывает понятную ошибку

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
