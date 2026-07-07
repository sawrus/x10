import { useQuery } from '@tanstack/react-query'
import { Link } from 'react-router-dom'

import { routes } from '../../../shared/config/routes'
import { StackOverview } from '../../../widgets/stack-overview'

export function DashboardPage() {
  const readiness = useQuery({
    queryKey: ['game-frontend-readiness'],
    queryFn: async () => ({ status: 'ready' }),
    staleTime: Number.POSITIVE_INFINITY,
  })

  return (
    <section className="rounded-[2rem] border border-cyan-300/20 bg-white/10 p-8 shadow-2xl shadow-cyan-950/40 backdrop-blur">
      <p className="mb-3 text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Epic 00</p>
      <div className="flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
        <div>
          <h2 className="text-4xl font-black tracking-tight md:text-6xl">X10 Game Frontend</h2>
          <p className="mt-4 max-w-2xl text-lg text-slate-300">
            Отдельный React/Vite skeleton для будущего игрового клиента. Админка остаётся в корне web, а игра
            разрабатывается изолированно в директории <code>web/game</code>.
          </p>
          <div className="mt-8 flex flex-wrap items-center gap-4 text-sm text-slate-300">
            <span>Query status: {readiness.data?.status ?? 'loading'}</span>
            <Link className="font-semibold text-cyan-300 hover:text-cyan-200" to={routes.quests}>
              Open quests route
            </Link>
          </div>
        </div>

        <div className="rounded-2xl border border-white/10 bg-slate-900/70 px-5 py-4 text-sm text-slate-300">
          Layout is now shared through AppShell and nested routes.
        </div>
      </div>

      <div className="mt-8">
        <StackOverview />
      </div>
    </section>
  )
}
