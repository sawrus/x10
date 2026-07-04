# TanStack Query provider

## Эпик

Epic 00

## Фича

Frontend Skeleton

## Контекст

Server state должен быть через TanStack Query.

## Задача для Codex

Добавь QueryClientProvider на уровне app. Настрой базовые параметры retry/staleTime. Добавь простой health query hook для проверки интеграции.

## Используемое API

- `GET /health`


## Ожидаемые файлы/зоны изменений

- `src/app/providers`
- `src/shared/api/health`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Query provider подключен
- [ ] Health hook работает

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
