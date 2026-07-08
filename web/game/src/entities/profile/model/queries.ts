import { queryOptions, useQuery, type UseQueryOptions, type UseQueryResult } from '@tanstack/react-query'

import { useAppStore } from '../../../app/store'
import type { ApiError } from '../../../shared/api'

import { getProfileDashboard } from './api'
import type { ProfileDashboard } from './types'

export const profileDashboardQueryKey = (profileId: string) => ['profile', profileId, 'dashboard'] as const

export function getProfileDashboardQueryOptions(profileId: string, actorId?: string) {
  return queryOptions<ProfileDashboard, ApiError, ProfileDashboard, ReturnType<typeof profileDashboardQueryKey>>({
    queryKey: profileDashboardQueryKey(profileId),
    queryFn: ({ signal }) =>
      getProfileDashboard(profileId, {
        actorId,
        signal,
      }),
  })
}

export function useProfileDashboardQuery(
  profileId: string,
  options?: Omit<
    UseQueryOptions<ProfileDashboard, ApiError, ProfileDashboard, ReturnType<typeof profileDashboardQueryKey>>,
    'queryFn' | 'queryKey'
  >,
): UseQueryResult<ProfileDashboard, Error> {
  const actorId = useAppStore((state) => state.profileId)
  const isScoped = Boolean(profileId) && actorId === profileId

  return useQuery<ProfileDashboard, ApiError, ProfileDashboard, ReturnType<typeof profileDashboardQueryKey>>({
    ...getProfileDashboardQueryOptions(profileId, isScoped ? actorId : undefined),
    ...options,
    enabled: (options?.enabled ?? true) && isScoped,
  }) as UseQueryResult<ProfileDashboard, Error>
}
