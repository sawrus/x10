import { Card, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

export function HistoryPage() {
  return (
    <Card>
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">History</p>
        <CardTitle className="mt-3">Progress history route is ready</CardTitle>
        <CardDescription className="mt-4">
        Здесь можно будет показать хронику завершённых дней, изменений опыта и важных игровых событий.
        </CardDescription>
      </CardHeader>
    </Card>
  )
}
