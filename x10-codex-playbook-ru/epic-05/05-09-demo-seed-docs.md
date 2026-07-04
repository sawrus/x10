# Demo seed docs

## Эпик

Epic 05

## Фича

Demo / Seed

## Контекст

Нужен способ быстро получить playable demo.

## Задача для Codex

Создай docs/demo-seed.md с инструкцией: как создать профиль, сферы, уровни, задачи через API/curl или UI. Не делай админку.

## Используемое API

- `POST /api/v2/profiles`
- `POST /api/v2/spheres`
- `POST /api/v2/tasks`
- `POST /api/v2/profiles/{profile_id}/levels`


## Ожидаемые файлы/зоны изменений

- `docs/demo-seed.md`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Инструкция понятна

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
