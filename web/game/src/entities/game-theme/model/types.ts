export type GameTheme = 'dendy' | 'apple'

export const gameThemes = ['dendy', 'apple'] as const satisfies readonly GameTheme[]
