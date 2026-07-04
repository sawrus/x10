# Phaser QA pass

## Эпик

Epic 04

## Фича

QA / Seed / Demo

## Контекст

После Phaser этапа проверить стабильность.

## Задача для Codex

Проверь Dashboard + GameMap: переходы страниц, выполнение квеста, повторные render, memory leak symptoms. Исправь найденное. Обнови docs/phaser-test-scenario.md.


## Ожидаемые файлы/зоны изменений

- `docs/phaser-test-scenario.md`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Build проходит
- [ ] GameMap стабилен

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
