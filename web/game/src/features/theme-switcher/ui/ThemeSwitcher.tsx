import { gameThemes, type GameTheme } from '../../../entities/game-theme'
import { useAppStore } from '../../../app/store'
import { Select } from '../../../shared/ui'

export function ThemeSwitcher() {
  const theme = useAppStore((state) => state.settings.theme)
  const setSetting = useAppStore((state) => state.setSetting)

  return (
    <div className="w-full max-w-[16rem]">
      <Select
        label="Theme"
        value={theme}
        onChange={(event) => setSetting('theme', event.target.value as GameTheme)}
        options={gameThemes.map((value) => ({
          label: value === 'dendy' ? 'Dendy' : 'Apple',
          value,
        }))}
      />
    </div>
  )
}
