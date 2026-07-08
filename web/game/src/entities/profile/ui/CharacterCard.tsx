import { Badge, Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../../shared/ui'

import type { Profile, ProfileDashboard } from '../model/types'

type CharacterCardProps = {
  avatarUrl?: string | null
  dashboard?: ProfileDashboard
  dashboardErrorMessage?: string | null
  profile: Profile
}

export function CharacterCard({ avatarUrl, dashboard, dashboardErrorMessage, profile }: CharacterCardProps) {
  const initials = getProfileInitials(profile.full_name)
  const progression = dashboard?.current

  return (
    <Card className="h-full border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/30">
      <CardHeader>
        <div className="flex flex-wrap items-start justify-between gap-4">
          <div>
            <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Character card</p>
            <CardTitle className="mt-3 text-4xl">{profile.full_name}</CardTitle>
            <CardDescription className="mt-4">
              Core identity snapshot for the active hero. This card stays reusable so later progression widgets can land
              here without forcing a page rewrite.
            </CardDescription>
          </div>
          <div
            aria-hidden="true"
            className="flex h-24 w-24 items-center justify-center overflow-hidden rounded-[1.75rem] border border-cyan-300/30 bg-cyan-300/10 text-3xl font-black text-cyan-100 shadow-lg shadow-cyan-950/20"
          >
            {avatarUrl ? <img alt={`${profile.full_name} avatar`} className="h-full w-full object-cover" src={avatarUrl} /> : initials}
          </div>
        </div>
      </CardHeader>

      <CardContent className="space-y-6">
        <div className="flex flex-wrap gap-3">
          <Badge tone={progression ? 'success' : 'muted'}>
            {progression ? `Level ${progression.current_level}` : 'Level pending'}
          </Badge>
          <Badge tone={progression ? 'success' : 'muted'}>
            {progression ? `Balance ${progression.balance_score}` : 'Balance unavailable'}
          </Badge>
          <Badge tone={avatarUrl ? 'success' : 'muted'}>{avatarUrl ? 'Avatar ready' : 'Avatar pending'}</Badge>
        </div>

        <div className="grid gap-3 sm:grid-cols-2">
          <InfoItem label="Occupation" value={profile.occupation} />
          <InfoItem label="Timezone" value={profile.timezone} />
          <InfoItem label="Birth date" value={profile.birth_date} />
          <InfoItem label="Telegram" value={profile.telegram ?? 'Not added'} />
          <InfoItem label="Email" value={profile.email ?? 'Not added'} />
          <InfoItem label="Profile id" value={profile.id} />
        </div>

        {dashboardErrorMessage ? (
          <div className="rounded-[1.5rem] border border-amber-300/30 bg-amber-300/10 px-4 py-3 text-sm text-amber-100">
            Progression snapshot is temporarily unavailable: {dashboardErrorMessage}
          </div>
        ) : null}
      </CardContent>
    </Card>
  )
}

function InfoItem({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-[1.5rem] border border-white/10 bg-slate-950/55 px-4 py-4">
      <p className="text-xs font-semibold uppercase tracking-[0.24em] text-slate-400">{label}</p>
      <p className="mt-2 break-words text-sm font-semibold text-slate-100">{value}</p>
    </div>
  )
}

function getProfileInitials(fullName: string): string {
  const tokens = fullName
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)

  if (tokens.length === 0) {
    return 'XP'
  }

  return tokens.map((token) => token[0]?.toUpperCase() ?? '').join('')
}
