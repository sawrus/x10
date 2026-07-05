# Frontend Audit — Epic 00 / Frontend Skeleton

## Scope

Цель проверки — зафиксировать текущую структуру X10 перед добавлением нового frontend, отдельно проверить наличие `frontend/web/game`, `package.json`, Vite/React-конфигурации и выбрать безопасное место для будущего frontend без изменения backend-бизнес-логики.

## Что уже есть

- Backend находится в корне репозитория как Rust/Cargo-проект: `Cargo.toml`, `Cargo.lock`, `src/**`, `migrations/**`.
- В корне есть `Makefile` с целями для backend и web: `build`, `fmt`, `lint`, `test`, `run`, `web-build`, `web-test`, `clean`.
- Существующий web-каталог расположен по пути `web/`, а не `frontend/web/`.
- В `web/package.json` уже есть npm-проект `x10-admin` со скриптами:
  - `npm run dev` → `vite`
  - `npm run build` → `vite build`
  - `npm run preview` → `vite preview`
- Текущий web stack — Vue 3 + Vuetify + Vite:
  - runtime dependencies: `vue`, `vue-i18n`, `vuetify`, `@mdi/font`
  - dev dependencies: `vite`, `@vitejs/plugin-vue`, `vite-plugin-vuetify`, `sass`, `playwright-core`
- Vite-конфигурация находится в `web/vite.config.js`, подключает `@vitejs/plugin-vue` и `vite-plugin-vuetify`, собирает output в `web/dist`.
- Текущая админская SPA реализована в `web/src/main.js` и `web/src/App.vue`.
- Есть статическая игровая заглушка `web/public/game/index.html`; после сборки она попадает в `web/dist/game/index.html` и открывается из админки ссылкой `/game`.
- В `web/src/app.js`, `web/src/app.css` и `web/src/index.html` есть более ранняя vanilla/progression UI-зона, но основной Vite entry использует `web/index.html` → `web/src/main.js`.
- В `web/node_modules/` уже установлены зависимости, а `web/package-lock.json` зафиксирован.
- Проектные frontend-ориентиры уже описаны в `docs/codex/01-frontend-architecture.md`, `docs/codex/02-coding-rules.md`, `docs/codex/03-ui-guidelines.md` и плане Epic 00.

## Что не найдено

- Каталог `frontend/` не найден.
- Каталог `frontend/web/game` не найден.
- Отдельный React frontend не найден.
- React/Vite конфигурация не найдена: нет `@vitejs/plugin-react`, `react`, `react-dom`, `vite.config.ts` или отдельного React entrypoint.
- Phaser-зависимость и Phaser game shell не найдены; сейчас существует только статическая HTML-заглушка игры в `web/public/game/index.html`.
- Отдельные frontend test/lint scripts в `web/package.json` не заданы; доступны только `dev`, `build`, `preview`, а smoke-проверка обёрнута в `web/test.sh` и `make web-test`.

## Риски и ограничения

- `web/` уже занят production-like админской Vue/Vuetify SPA. Перевод этого каталога на React без явной миграции затронет существующую админку.
- `/game` уже используется как URL для игровой заглушки, но физически это `web/public/game/index.html`, а не отдельное приложение.
- Если новый React frontend должен стать основным пользовательским клиентом, нужно заранее решить, сосуществует ли он с Vue admin или заменяет текущий `web/`.
- Backend-бизнес-логика для этой проверки не менялась.

## Рекомендация по размещению нового frontend

Лучшее размещение зависит от роли нового клиента:

1. **Если нужен отдельный игровой React/Phaser клиент рядом с текущей админкой:** создать новый каталог `frontend/web/` или `frontend/game/` и держать его отдельным npm workspace/package. Это минимизирует риск сломать существующий `web/` admin и позволит отдельно подключить React, Router, Query, Zustand и Phaser.
2. **Если новый frontend должен постепенно заменить текущий root web:** использовать текущий `web/` только после отдельного решения о миграции Vue → React. Для Epic 00 безопаснее сначала не переписывать `web/src/App.vue`, а подготовить отдельный React skeleton.
3. **Для маршрута игры:** сохранить публичный URL `/game`, но на следующем шаге заменить статическую `web/public/game/index.html` на собранный game bundle или настроить backend/static serving для нового `frontend/game` artifact.

Итого: для следующей задачи Frontend Skeleton безопаснее размещать новый пользовательский React frontend вне существующей Vue-админки — например, в `frontend/web/` или `frontend/game/` — а текущий `web/` оставить как admin UI до явного решения о миграции.

## Проверки

- `make web-test` — прошёл; выполняет сборку web и smoke-проверки `web/dist/index.html`, `web/dist/game/index.html`, JS/CSS assets и текста `x10 admin`.
- `make test` — прошёл; включает `web-test` и Rust test suite.
