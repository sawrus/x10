import type { GameTheme } from './game-theme'

export type ProfileId = string & { readonly __brand: 'ProfileId' }

export type SphereId = string

export type AppModal = 'none' | 'day-summary' | 'health-details' | 'quest-log' | 'settings'

export type AppUiState = {
  readonly activeModal: AppModal
  readonly highlightedSphereId: SphereId | null
  readonly isHudVisible: boolean
  readonly showHealthDetails: boolean
}

export type GameSettings = {
  readonly animationsEnabled: boolean
  readonly autoFinalizeDay: boolean
  readonly compactMode: boolean
  readonly hapticsEnabled: boolean
  readonly musicEnabled: boolean
  readonly soundEnabled: boolean
  readonly theme: GameTheme
}

export type AppStateSnapshot = {
  readonly profileId: ProfileId | null
  readonly settings: GameSettings
  readonly ui: AppUiState
}

export function createDefaultAppUiState(): AppUiState {
  return {
    activeModal: 'none',
    highlightedSphereId: null,
    isHudVisible: true,
    showHealthDetails: false,
  }
}

export function createDefaultGameSettings(): GameSettings {
  return {
    animationsEnabled: true,
    autoFinalizeDay: false,
    compactMode: false,
    hapticsEnabled: true,
    musicEnabled: true,
    soundEnabled: true,
    theme: 'dendy',
  }
}
