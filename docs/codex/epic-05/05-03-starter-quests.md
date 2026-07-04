# Starter quests

## Эпик

Epic 05

## Фича

Стартовый контент

## Контекст

Нужны стартовые задания.

## Задача для Codex

Добавь starter quest pack минимум 50 заданий с positive/negative, sphere mapping, planned params. Дай пользователю выбрать 3-7 на старт.

## Используемое API

- `POST /api/v2/tasks`


## Ожидаемые файлы/зоны изменений

- `src/features/onboarding/starter-quests`
- `docs/starter-content.md`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Есть 50 заданий
- [ ] Можно создать выбранные

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
