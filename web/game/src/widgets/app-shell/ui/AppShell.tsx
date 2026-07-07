import { NavLink, Outlet } from 'react-router-dom'

import { ThemeSwitcher } from '../../../features/theme-switcher'
import { routes } from '../../../shared/config/routes'

type NavigationItem = {
  label: string
  to: string
  end?: boolean
}

const navigationItems: NavigationItem[] = [
  { label: 'Dashboard', to: routes.dashboard, end: true },
  { label: 'Quests', to: routes.quests },
  { label: 'Character', to: routes.character },
  { label: 'History', to: routes.history },
  { label: 'Onboarding', to: routes.onboarding },
]

export function AppShell() {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <div className="mx-auto flex min-h-screen w-full max-w-6xl flex-col px-4 py-6 sm:px-6 lg:px-8">
        <header className="rounded-[2rem] border border-cyan-300/20 bg-slate-900/75 px-5 py-5 shadow-2xl shadow-cyan-950/30 backdrop-blur">
          <div className="flex flex-col gap-5 lg:flex-row lg:items-center lg:justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-[0.35em] text-cyan-300">X10 Life RPG</p>
              <h1 className="mt-2 text-2xl font-black tracking-tight sm:text-3xl">Game navigation shell</h1>
            </div>
            <ThemeSwitcher />
          </div>

          <nav className="mt-5 flex flex-wrap gap-3">
            {navigationItems.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
                end={item.end}
                className={({ isActive }) =>
                  [
                    'rounded-full px-4 py-2 text-sm font-semibold transition-colors',
                    isActive
                      ? 'bg-cyan-300 text-slate-950'
                      : 'border border-white/10 bg-slate-800/80 text-slate-200 hover:border-cyan-300/40 hover:text-white',
                  ].join(' ')
                }
              >
                {item.label}
              </NavLink>
            ))}
          </nav>
        </header>

        <main className="flex-1 py-6">
          <Outlet />
        </main>
      </div>
    </div>
  )
}
