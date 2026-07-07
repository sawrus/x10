import { Card, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

export function QuestsPage() {
  return (
    <Card>
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Quests</p>
        <CardTitle className="mt-3">Quest board route is wired</CardTitle>
        <CardDescription className="mt-4">
        Здесь появятся активные квесты, категории задач и игровые действия без смешивания с Phaser-слоем.
        </CardDescription>
      </CardHeader>
    </Card>
  )
}
