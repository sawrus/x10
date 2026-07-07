import { Card, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

export function OnboardingPage() {
  return (
    <Card>
      <CardHeader>
        <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Onboarding</p>
        <CardTitle className="mt-3">Starter flow entrypoint</CardTitle>
        <CardDescription className="mt-4">
        Роут для первого запуска уже на месте и готов принять сценарий знакомства с Life RPG циклом.
        </CardDescription>
      </CardHeader>
    </Card>
  )
}
