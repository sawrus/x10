# React GameMap component

## Эпик

Epic 04

## Фича

Игровая карта Phaser

## Контекст

React должен монтировать Phaser аккуратно.

## Задача для Codex

Создай GameMap React component, который создаёт/уничтожает Phaser instance без утечек. Компонент принимает profile/dashboard props.


## Ожидаемые файлы/зоны изменений

- `src/widgets/game-map/GameMap.tsx`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] GameMap монтируется
- [ ] Нет повторного создания при каждом render

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
