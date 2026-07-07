import { createBrowserRouter } from 'react-router-dom'

import { DashboardPage } from '../../pages/dashboard'
import { CharacterPage } from '../../pages/character'
import { HistoryPage } from '../../pages/history'
import { OnboardingPage } from '../../pages/onboarding'
import { QuestsPage } from '../../pages/quests'
import { AppShell } from '../../widgets/app-shell'

export const router = createBrowserRouter(
  [
    {
      path: '/',
      element: <AppShell />,
      children: [
        {
          index: true,
          element: <DashboardPage />,
        },
        {
          path: 'quests',
          element: <QuestsPage />,
        },
        {
          path: 'character',
          element: <CharacterPage />,
        },
        {
          path: 'history',
          element: <HistoryPage />,
        },
        {
          path: 'onboarding',
          element: <OnboardingPage />,
        },
      ],
    },
  ],
  { basename: '/game' },
)
