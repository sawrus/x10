import type { ProfileId, SphereId } from './app-state'

export type QuestCompletedEvent = {
  readonly profileId: ProfileId
  readonly questId: string
  readonly xpReward: number
  readonly balanceReward: number
}

export type BalanceChangedEvent = {
  readonly profileId: ProfileId
  readonly previousBalance: number
  readonly nextBalance: number
  readonly delta: number
  readonly reason?: string
}

export type LevelUpEvent = {
  readonly profileId: ProfileId
  readonly previousLevel: number
  readonly nextLevel: number
}

export type DayFinalizedEvent = {
  readonly profileId: ProfileId
  readonly dayNumber: number
  readonly completedQuestIds: readonly string[]
}

export type SphereHighlightEvent = {
  readonly active: boolean
  readonly profileId?: ProfileId
  readonly reason?: string
  readonly sphereId: SphereId
  readonly source?: 'hud' | 'map' | 'quest-log' | 'system'
}

export type GameEventMap = {
  'quest:completed': QuestCompletedEvent
  'balance:changed': BalanceChangedEvent
  'level:up': LevelUpEvent
  'day:finalized': DayFinalizedEvent
  'sphere:highlight': SphereHighlightEvent
}
