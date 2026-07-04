# Level Progress UI

## Эпик

Epic 02

## Фича

Баланс и уровни

## Контекст

Игрок должен видеть прогресс до следующего уровня.

## Задача для Codex

Создай LevelProgress компонент: current balance, current level, next level, процент прогресса. Данные брать из dashboard/levels/balances где доступно.

## Используемое API

- `GET /api/v2/profiles/{profile_id}/dashboard`
- `GET /api/v2/profiles/{profile_id}/levels`


## Ожидаемые файлы/зоны изменений

- `src/entities/level/ui`
- `src/widgets`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Прогрессбар работает
- [ ] Корректно обрабатывает max level

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
