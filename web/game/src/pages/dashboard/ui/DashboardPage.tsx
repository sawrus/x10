import { useQuery } from '@tanstack/react-query'
import { Link } from 'react-router-dom'

import { ThemeSwitcher } from '../../../features/theme-switcher'
import { routes } from '../../../shared/config/routes'
import { StackOverview } from '../../../widgets/stack-overview'

export function DashboardPage() {
  const readiness = useQuery({
    queryKey: ['game-frontend-readiness'],
    queryFn: async () => ({ status: 'ready' }),
    staleTime: Number.POSITIVE_INFINITY,
  })

  return (
    <main className="min-h-screen bg-slate-950 text-slate-100">
      <section className="mx-auto flex min-h-screen w-full max-w-5xl flex-col justify-center px-6 py-16">
        <p className="mb-3 text-sm font-semibold uppercase tracking-[0.3em] text-cyan-300">Epic 00</p>
        <div className="rounded-3xl border border-cyan-300/20 bg-white/10 p-8 shadow-2xl shadow-cyan-950/40 backdrop-blur">
          <div className="flex flex-col gap-6 md:flex-row md:items-start md:justify-between">
            <div>
              <h1 className="text-4xl font-black tracking-tight md:text-6xl">X10 Game Frontend</h1>
              <p className="mt-4 max-w-2xl text-lg text-slate-300">
                Отдельный React/Vite skeleton для будущего игрового клиента. Админка остаётся в корне web,
                а игра разрабатывается изолированно в директории <code>web/game</code>.
              </p>
            </div>
            <ThemeSwitcher />
          </div>

          <div className="mt-8">
            <StackOverview />
          </div>

          <div className="mt-8 flex flex-wrap items-center gap-4 text-sm text-slate-300">
            <span>Query status: {readiness.data?.status ?? 'loading'}</span>
            <Link className="font-semibold text-cyan-300 hover:text-cyan-200" to={routes.dashboard}>
              Game home route
            </Link>
          </div>
        </div>
      </section>
    </main>
  )
}
