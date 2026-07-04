# Cadence helpers

## Эпик

Epic 03

## Фича

Повторяемые задания

## Контекст

Нужно корректно понимать daily/weekly/monthly/one-time задания на frontend.

## Задача для Codex

Создай helper функции для cadence: определение периода, доступность сегодня, label, next availability. Учти timezone профиля.


## Ожидаемые файлы/зоны изменений

- `src/entities/task/lib/cadence.ts`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Helpers покрыты тестами или примерами

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
