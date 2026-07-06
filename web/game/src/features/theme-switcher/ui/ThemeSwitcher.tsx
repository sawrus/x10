import type { GameTheme } from '../../../entities/game-theme'
import { useGameThemeStore } from '../../../entities/game-theme'

export function ThemeSwitcher() {
  const { theme, setTheme } = useGameThemeStore()

  return (
    <label className="flex items-center gap-3 rounded-2xl border border-white/10 bg-slate-900/80 px-4 py-3">
      <span className="text-sm text-slate-300">Theme</span>
      <select
        className="rounded-lg bg-slate-800 px-3 py-2 text-sm text-white"
        value={theme}
        onChange={(event) => setTheme(event.target.value as GameTheme)}
      >
        <option value="dendy">Dendy</option>
        <option value="apple">Apple</option>
      </select>
    </label>
  )
}
