import { apiClient, type ApiRequestOptions } from '../../../shared/api'

import type { ProfileDashboard } from './types'

function requireProfileActorId(profileId: string, options?: ApiRequestOptions): ApiRequestOptions {
  const actorId = options?.actorId?.trim()

  if (!actorId) {
    throw new Error(`Profile dashboard request for "${profileId}" requires X-Actor-Id.`)
  }

  return {
    ...options,
    actorId,
  }
}

export function getProfileDashboard(profileId: string, options?: ApiRequestOptions) {
  return apiClient.get<ProfileDashboard>(`/api/v2/profiles/${profileId}/dashboard`, requireProfileActorId(profileId, options))
}
