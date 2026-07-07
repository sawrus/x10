import { useMemo, useState } from 'react'
import { Link } from 'react-router-dom'

import { useAppStore } from '../../../app/store'
import { gameEventBus } from '../../../shared/lib/game-event-bus'
import { isApiError, useHealthQuery } from '../../../shared/api'
import { routes } from '../../../shared/config/routes'
import { Badge, Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Modal, ProgressBar } from '../../../shared/ui'
import { StackOverview } from '../../../widgets/stack-overview'

export function DashboardPage() {
  const [draftProfileId, setDraftProfileId] = useState('')
  const clearProfileId = useAppStore((state) => state.clearProfileId)
  const closeModal = useAppStore((state) => state.closeModal)
  const currentProfileId = useAppStore((state) => state.profileId)
  const activeModal = useAppStore((state) => state.ui.activeModal)
  const openModal = useAppStore((state) => state.openModal)
  const setProfileId = useAppStore((state) => state.setProfileId)
  const setShowHealthDetails = useAppStore((state) => state.setShowHealthDetails)
  const showHealthDetails = useAppStore((state) => state.ui.showHealthDetails)
  const readiness = useHealthQuery()
  const queryStatus = readiness.isError
    ? isApiError(readiness.error)
      ? `error ${readiness.error.status}`
      : 'error'
    : readiness.data?.status ?? 'loading'
  const queryTone = readiness.isError ? 'danger' : readiness.isSuccess ? 'success' : 'muted'
  const completionValue = 5 + Number(readiness.isSuccess) + Number(Boolean(currentProfileId)) + Number(showHealthDetails)
  const healthDetails = useMemo(
    () => [
      ['status', queryStatus],
      ['cached', readiness.isFetched ? 'yes' : 'no'],
      ['profile', currentProfileId ?? 'not selected'],
    ],
    [currentProfileId, queryStatus, readiness.isFetched],
  )

  return (
    <>
      <Card className="border-cyan-300/20 bg-white/10 shadow-2xl shadow-cyan-950/40">
        <CardHeader>
          <p className="text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Epic 00</p>
          <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
            <div>
              <CardTitle className="text-4xl md:text-6xl">X10 Game Frontend</CardTitle>
              <CardDescription className="mt-4 text-lg">
                Отдельный React/Vite skeleton для будущего игрового клиента. Админка остаётся в корне `web`, а игра
                развивается изолированно внутри `web/game`.
              </CardDescription>
            </div>

            <Card className="min-w-[16rem] rounded-[1.5rem] bg-slate-950/60 p-5">
              <Badge tone={queryTone}>Health {queryStatus}</Badge>
              <ProgressBar className="mt-4" label="Epic 00 completion" max={10} value={completionValue} />
            </Card>
          </div>
        </CardHeader>

        <CardContent className="grid gap-6 lg:grid-cols-[1.4fr,0.9fr]">
          <div className="space-y-6">
            <Card className="rounded-[1.5rem] bg-slate-950/45">
              <CardHeader>
                <p className="text-sm font-semibold uppercase tracking-[0.25em] text-cyan-300">Foundations</p>
                <CardDescription>
                  Query provider, app store, shared primitives и event contract теперь собираются как единая база для
                  следующих игровых эпиков.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <StackOverview />
              </CardContent>
            </Card>

            <Card className="rounded-[1.5rem] bg-slate-950/45">
              <CardHeader>
                <p className="text-sm font-semibold uppercase tracking-[0.25em] text-cyan-300">Profile Context</p>
                <CardDescription>
                  Client state хранит только локальный контекст и настройки. Server state остаётся в TanStack Query.
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <Input
                  label="Current profile id"
                  placeholder="profile-uuid"
                  value={draftProfileId}
                  onChange={(event) => setDraftProfileId(event.target.value)}
                  hint={currentProfileId ? `Saved profile: ${currentProfileId}` : 'No profile selected yet'}
                />
              </CardContent>
              <CardFooter>
                <Button
                  onClick={() => {
                    setProfileId(draftProfileId)
                    if (draftProfileId.trim()) {
                      gameEventBus.emit('sphere:highlight', {
                        active: true,
                        reason: 'dashboard-preview',
                        source: 'system',
                        sphereId: 'focus',
                      })
                    }
                  }}
                >
                  Save profile context
                </Button>
                <Button variant="secondary" onClick={clearProfileId}>
                  Clear
                </Button>
              </CardFooter>
            </Card>
          </div>

          <Card className="rounded-[1.5rem] bg-slate-950/45">
            <CardHeader>
              <p className="text-sm font-semibold uppercase tracking-[0.25em] text-cyan-300">Navigation Ready</p>
              <CardDescription>
                Базовые маршруты уже подключены и готовы принять доменные widgets без смешивания с Phaser-слоем.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex flex-wrap gap-3 text-sm text-slate-300">
                <Badge tone={queryTone}>query {queryStatus}</Badge>
                <Badge>{currentProfileId ? 'profile selected' : 'profile pending'}</Badge>
              </div>
              <div className="flex flex-col gap-3">
                <Link className="font-semibold text-cyan-300 hover:text-cyan-200" to={routes.quests}>
                  Open quests route
                </Link>
                <Link className="font-semibold text-cyan-300 hover:text-cyan-200" to={routes.character}>
                  Open character route
                </Link>
              </div>
            </CardContent>
            <CardFooter>
              <Button variant="secondary" onClick={() => setShowHealthDetails(!showHealthDetails)}>
                {showHealthDetails ? 'Hide health details' : 'Show health details'}
              </Button>
              <Button variant="ghost" onClick={() => openModal('health-details')}>
                Preview modal
              </Button>
            </CardFooter>
          </Card>
        </CardContent>
      </Card>

      <Modal
        open={activeModal === 'health-details'}
        onClose={closeModal}
        title="Health integration details"
        description="Simple verification view for the shared health query hook and local app store."
        footer={<Button onClick={closeModal}>Close panel</Button>}
      >
        <div className="space-y-3">
          {healthDetails.map(([label, value]) => (
            <div key={label} className="flex items-center justify-between gap-4 rounded-2xl border border-white/10 bg-slate-950/70 px-4 py-3">
              <span className="text-slate-400">{label}</span>
              <span className="font-semibold text-slate-100">{value}</span>
            </div>
          ))}
        </div>
      </Modal>
    </>
  )
}
