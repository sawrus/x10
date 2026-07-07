import { create } from 'zustand'

import {
  createDefaultAppUiState,
  createDefaultGameSettings,
  type AppModal,
  type AppStateSnapshot,
  type AppUiState,
  type GameSettings,
  type ProfileId,
  type SphereId,
} from '../../shared/model/app-state'

type BooleanUiFlag = {
  [TKey in keyof AppUiState]: AppUiState[TKey] extends boolean ? TKey : never
}[keyof AppUiState]

export type AppStore = AppStateSnapshot & {
  clearProfileId: () => void
  closeModal: () => void
  openModal: (modal: Exclude<AppModal, 'none'>) => void
  patchSettings: (settings: Partial<GameSettings>) => void
  patchUi: (ui: Partial<AppUiState>) => void
  resetSettings: () => void
  resetUi: () => void
  setHighlightedSphereId: (sphereId: SphereId | null) => void
  setProfileId: (profileId: ProfileId | string) => void
  setSetting: <TKey extends keyof GameSettings>(key: TKey, value: GameSettings[TKey]) => void
  setShowHealthDetails: (value: boolean) => void
  setUiFlag: <TKey extends BooleanUiFlag>(key: TKey, value: AppUiState[TKey]) => void
}

function normalizeProfileId(profileId: ProfileId | string): ProfileId | null {
  const normalizedProfileId = profileId.trim()

  return normalizedProfileId ? (normalizedProfileId as ProfileId) : null
}

export const useAppStore = create<AppStore>((set) => ({
  profileId: null,
  settings: createDefaultGameSettings(),
  ui: createDefaultAppUiState(),
  clearProfileId: () => set({ profileId: null }),
  closeModal: () =>
    set((state) => ({
      ui: {
        ...state.ui,
        activeModal: 'none',
      },
    })),
  openModal: (modal) =>
    set((state) => ({
      ui: {
        ...state.ui,
        activeModal: modal,
      },
    })),
  patchSettings: (settings) =>
    set((state) => ({
      settings: {
        ...state.settings,
        ...settings,
      },
    })),
  patchUi: (ui) =>
    set((state) => ({
      ui: {
        ...state.ui,
        ...ui,
      },
    })),
  resetSettings: () => set({ settings: createDefaultGameSettings() }),
  resetUi: () => set({ ui: createDefaultAppUiState() }),
  setHighlightedSphereId: (sphereId) =>
    set((state) => ({
      ui: {
        ...state.ui,
        highlightedSphereId: sphereId,
      },
    })),
  setProfileId: (profileId) => set({ profileId: normalizeProfileId(profileId) }),
  setSetting: (key, value) =>
    set((state) => ({
      settings: {
        ...state.settings,
        [key]: value,
      },
    })),
  setShowHealthDetails: (value) =>
    set((state) => ({
      ui: {
        ...state.ui,
        showHealthDetails: value,
      },
    })),
  setUiFlag: (key, value) =>
    set((state) => ({
      ui: {
        ...state.ui,
        [key]: value,
      },
    })),
}))
