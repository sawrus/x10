# Базовый API client

## Эпик

Epic 00

## Фича

Frontend Skeleton

## Контекст

Frontend должен ходить в существующее Rust API.

## Задача для Codex

Создай базовый HTTP client с `baseUrl` из env. Добавь поддержку JSON, ошибок и header `X-Actor-Id`. Не используй fetch напрямую в компонентах.

## Используемое API

- `GET /health`


## Ожидаемые файлы/зоны изменений

- `src/shared/api`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] Есть apiClient
- [ ] Ошибки API типизированы
- [ ] X-Actor-Id поддержан

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
