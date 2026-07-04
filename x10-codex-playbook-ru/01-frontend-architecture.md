# Архитектура frontend

## Рекомендуемая структура

```text
src/
  app/
    providers/
    router/
    styles/
  pages/
    dashboard/
    quests/
    character/
    history/
    onboarding/
  widgets/
    app-shell/
    game-map/
    quest-board/
    daily-summary/
  features/
    profile/
    spheres/
    quests/
    executions/
    balances/
    levels/
    days/
    onboarding/
    game-feedback/
  entities/
    profile/
    sphere/
    task/
    execution/
    balance/
    level/
    day/
  shared/
    api/
    config/
    lib/
    ui/
    types/
```

## Правила

- `shared` не знает о бизнес-фичах.
- `entities` содержат типы, маленькие UI-компоненты и model helpers.
- `features` содержат пользовательские действия.
- `widgets` собирают несколько features/entities.
- `pages` собирают экран.
- `app` содержит провайдеры и router.

## API

- Все HTTP-запросы держать в `shared/api`.
- Запросы домена оборачивать в hooks внутри features/entities.
- В компонентах не писать `fetch` напрямую.
