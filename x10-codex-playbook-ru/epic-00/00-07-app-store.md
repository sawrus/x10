# Zustand app store

## Эпик

Epic 00

## Фича

Frontend Skeleton

## Контекст

Нужен client state для текущего профиля и UI.

## Задача для Codex

Создай Zustand store для выбранного `profileId`, UI flags и game settings. Не храни в нём данные, приходящие с backend списками.


## Ожидаемые файлы/зоны изменений

- `src/shared/model`
- `src/app/store`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] profileId можно установить
- [ ] Server state не хранится в Zustand

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
