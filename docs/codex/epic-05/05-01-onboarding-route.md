# Onboarding route

## Эпик

Epic 05

## Фича

Онбординг

## Контекст

Новому игроку нужен быстрый первый опыт.

## Задача для Codex

Создай onboarding route/page с шагами: профиль → сферы → первые квесты → первое выполнение.

## Используемое API

- `POST /api/v2/profiles`
- `POST /api/v2/spheres`
- `POST /api/v2/tasks`


## Ожидаемые файлы/зоны изменений

- `src/pages/onboarding`
- `src/features/onboarding`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Onboarding route есть

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
