import { Card, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

export function CharacterPage() {
  return (
    <Card>
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character</p>
        <CardTitle className="mt-3">Hero profile will live here</CardTitle>
        <CardDescription className="mt-4">
        Базовый маршрут готов для карточки персонажа, прогресса и будущих игровых характеристик.
        </CardDescription>
      </CardHeader>
    </Card>
  )
}
