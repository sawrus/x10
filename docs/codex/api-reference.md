# API Reference для Codex

Источник: `src/api/routes.rs` проекта `sawrus/x10`.

## Общие правила

- Базовый префикс API: `/api/v2`
- Для защищённых операций использовать header: `X-Actor-Id: <profile_id>`
- Backend на Rust является источником истины.
- Frontend не должен дублировать бизнес-логику backend.
- Для server state использовать TanStack Query.
- Для client/game state использовать Zustand/Event Bus.

## Health / Metrics

- `GET /health`
- `GET /metrics`

## Profiles

- `GET /api/v2/profiles`
- `POST /api/v2/profiles`
- `GET /api/v2/profiles/{profile_id}`
- `PATCH /api/v2/profiles/{profile_id}`
- profile responses now include `created_at` and `updated_at` RFC3339 timestamps so the game roster can show when a hero was created or last edited

### CreateProfilePayload

```ts
{
  full_name: string
  birth_date: string
  occupation: string
  telegram?: string | null
  email?: string | null
  timezone: string
}
```

## Profile photos

- `POST /api/v2/profiles/{profile_id}/photos`
- `GET /api/v2/profiles/{profile_id}/photos`
- `GET /api/v2/photos/{photo_id}`
- `DELETE /api/v2/photos/{photo_id}`
- `POST /api/v2/profiles/{profile_id}/photos/{photo_id}/select`

## Spheres

- `GET /api/v2/spheres`
- `POST /api/v2/spheres`
- `GET /api/v2/spheres/{sphere_id}`
- `PATCH /api/v2/spheres/{sphere_id}`
- `DELETE /api/v2/spheres/{sphere_id}`

### CreateSpherePayload

```ts
{
  name: string
  weight: number
}
```

## Tasks / Quests

- `POST /api/v2/tasks`
- `GET /api/v2/profiles/{profile_id}/tasks`
- `GET /api/v2/tasks/{task_id}`
- `PATCH /api/v2/tasks/{task_id}`
- `DELETE /api/v2/tasks/{task_id}`

### CreateTaskPayload

```ts
{
  profile_id: string
  title: string
  sphere_id?: string | null
  kind: "positive" | "negative" // проверить реальные enum values в generated/openapi/types
  planned_weight: number
  planned_score: number
  planned_rate: number
  cadence: string
  starts_on: string
}
```

## Task executions

- `POST /api/v2/tasks/{task_id}/executions`
- `GET /api/v2/profiles/{profile_id}/executions`
- `GET /api/v2/executions/{execution_id}`
- `DELETE /api/v2/executions/{execution_id}`

### CreateTaskExecutionPayload

```ts
{
  actual_score: number
  actual_rate: number
  completed_at?: string | null
}
```

## Balances

- `GET /api/v2/profiles/{profile_id}/balances`

## Dashboard

- `GET /api/v2/profiles/{profile_id}/dashboard`

## Levels

- `GET /api/v2/profiles/{profile_id}/levels`
- `POST /api/v2/profiles/{profile_id}/levels`
- `PATCH /api/v2/levels/{level_id}`
- `DELETE /api/v2/levels/{level_id}`

### CreateLevelPayload

```ts
{
  code: string
  ordinal: number
  min_balance: number
  target_planned_score: number
  target_planned_rate: number
}
```

## Day finalizations

- `POST /api/v2/profiles/{profile_id}/days/{date}/finalize`
- `GET /api/v2/profiles/{profile_id}/days/finalizations`
- `DELETE /api/v2/day-finalizations/{finalization_id}`

### CreateDayFinalizationPayload

```ts
{
  note?: string | null
}
```
