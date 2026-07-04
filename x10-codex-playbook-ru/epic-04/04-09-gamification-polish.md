# Полировка геймификации

## Эпик

Epic 04

## Фича

Gamification Polish

## Контекст

Нужно добавить ощущение игры поверх UI.

## Задача для Codex

Добавь badges/placeholders, quest rarity visual, difficulty labels, приятные микрокопи и иконки в карточках.


## Ожидаемые файлы/зоны изменений

- `src/entities/task/ui`
- `src/shared/ui`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] UI выглядит как Life RPG

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
