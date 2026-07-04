# Установка frontend stack

## Эпик

Epic 00

## Фича

Frontend Skeleton

## Контекст

Нужно подготовить React/Vite frontend для игры.

## Задача для Codex

Если frontend ещё не настроен, добавь Vite + React + TypeScript. Установи TanStack Query, Zustand, React Router, Tailwind CSS. Если frontend уже есть — аккуратно донастрой недостающие зависимости.


## Ожидаемые файлы/зоны изменений

- `package.json`
- `vite.config.ts`
- `tsconfig*.json`
- `src/`


## Что НЕ делать

- Не переписывать backend без прямой необходимости.
- Не придумывать новые API endpoints, если можно использовать существующие из api-reference.md.
- Не помещать бизнес-логику в Phaser.
- Не делать большую соседнюю фичу в рамках этого prompt.

## Definition of Done

- [ ] npm run build проходит
- [ ] Зависимости добавлены минимально

## Перед завершением

- Запусти доступные проверки проекта.
- Если есть frontend: `npm run build`.
- Не оставляй TODO без необходимости.
- Обнови `docs/codex/PROGRESS.md` или локальный progress-файл, если он есть в репозитории.
