# Dashboard MVP

## Эпик

Epic 02

## Фича

Главный экран

## Контекст

Главный экран должен собрать персонажа, квесты и прогресс.

## Задача для Codex

Собери Dashboard: CharacterCard, LevelProgress, today's/active quests, recent balance/executions if available. Добавь CTA выполнить квест.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/dashboard`
- `GET /api/v2/profiles/{profile_id}/tasks`


## Ожидаемые файлы/зоны изменений

- `src/pages/dashboard`
- `src/widgets`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Dashboard пригоден для игры

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
