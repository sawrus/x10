import { create } from 'zustand'

type GameTheme = 'dendy' | 'apple'

type AppState = {
  theme: GameTheme
  setTheme: (theme: GameTheme) => void
}

export const useAppStore = create<AppState>((set) => ({
  theme: 'dendy',
  setTheme: (theme) => set({ theme }),
}))
