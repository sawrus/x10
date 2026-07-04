# Базовый роутер и layout

## Эпик

Epic 00

## Фича

Frontend Skeleton

## Контекст

Нужна базовая навигация игры.

## Задача для Codex

Настрой React Router. Добавь маршруты: `/game`, `/game/quests`, `/game/character`, `/game/history`, `/game/onboarding`. Создай AppShell с навигацией.


## Ожидаемые файлы/зоны изменений

- `src/app/router`
- `src/widgets/app-shell`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Маршруты открываются
- [ ] Есть общий layout

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
