import { Link } from 'react-router-dom'

import { CharacterCard, ProfileEditorForm, type Profile, type ProfileDashboard, type UpdateProfileDto } from '../../../entities/profile'
import { routes } from '../../../shared/config/routes'
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

type CharacterProfileContentProps =
  | {
      kind: 'empty'
      onOpenDashboard?: string
    }
  | {
      kind: 'error'
      message: string
      onRetry: () => void
    }
  | {
      kind: 'loading'
    }
  | {
      kind: 'ready'
      dashboard?: ProfileDashboard
      dashboardErrorMessage?: string | null
      isSaving: boolean
      onSubmit: (payload: UpdateProfileDto) => Promise<unknown>
      profile: Profile
      submitError?: unknown
    }

export function CharacterProfileContent(props: CharacterProfileContentProps) {
  if (props.kind === 'empty') {
    return (
      <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/30">
        <CardHeader>
          <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character page</p>
          <CardTitle className="mt-3">Select a profile to unlock the hero sheet</CardTitle>
          <CardDescription className="mt-4">
            Character UI is ready, but it needs an active profile id from the dashboard before it can load the current
            hero.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Link
            className="inline-flex items-center justify-center rounded-full bg-cyan-300 px-4 py-2.5 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-300/80 focus-visible:ring-offset-2 focus-visible:ring-offset-slate-950"
            to={props.onOpenDashboard ?? routes.dashboard}
          >
            Open dashboard
          </Link>
        </CardContent>
      </Card>
    )
  }

  if (props.kind === 'loading') {
    return (
      <div className="grid gap-6 xl:grid-cols-[1.1fr,0.9fr]">
        <LoadingCard />
        <LoadingCard />
      </div>
    )
  }

  if (props.kind === 'error') {
    return (
      <Card className="border-rose-400/30 bg-rose-400/10 shadow-2xl shadow-rose-950/20">
        <CardHeader>
          <p className="text-sm font-semibold uppercase tracking-[0.3em] text-rose-200">Character page</p>
          <CardTitle className="mt-3">Profile failed to load</CardTitle>
          <CardDescription className="mt-4 text-rose-100">{props.message}</CardDescription>
        </CardHeader>
        <CardContent>
          <Button variant="secondary" onClick={props.onRetry}>
            Retry
          </Button>
        </CardContent>
      </Card>
    )
  }

  return (
    <div className="grid gap-6 xl:grid-cols-[1.1fr,0.9fr]">
      <CharacterCard dashboard={props.dashboard} dashboardErrorMessage={props.dashboardErrorMessage} profile={props.profile} />
      <ProfileEditorForm
        isSaving={props.isSaving}
        onSubmit={props.onSubmit}
        profile={props.profile}
        submitError={props.submitError}
      />
    </div>
  )
}

function LoadingCard() {
  return (
    <Card className="h-full animate-pulse border-white/10 bg-slate-900/70">
      <CardHeader>
        <div className="h-4 w-32 rounded-full bg-white/10" />
        <div className="h-10 w-3/4 rounded-full bg-white/10" />
        <div className="h-4 w-full rounded-full bg-white/10" />
        <div className="h-4 w-2/3 rounded-full bg-white/10" />
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid gap-3 sm:grid-cols-2">
          {Array.from({ length: 6 }).map((_, index) => (
            <div key={index} className="h-20 rounded-[1.5rem] bg-white/10" />
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
