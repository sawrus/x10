import { create } from 'zustand'

import type { GameTheme } from './types'

type GameThemeState = {
  theme: GameTheme
  setTheme: (theme: GameTheme) => void
}

export const useGameThemeStore = create<GameThemeState>((set) => ({
  theme: 'dendy',
  setTheme: (theme) => set({ theme }),
}))
