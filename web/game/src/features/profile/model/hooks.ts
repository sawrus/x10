import { useMutation, useQueryClient, type UseMutationResult, type UseQueryResult } from '@tanstack/react-query'

import { useAppStore } from '../../../app/store'
import type { CreateProfileDto, Profile, UpdateProfileDto } from '../../../entities/profile/model/types'
import type {
  UseCreateProfileApiMutationOptions,
  UseProfileApiQueryOptions,
  UseUpdateProfileApiMutationOptions,
} from '../../../shared/api'
import {
  profileQueryKey,
  updateProfile,
  useCreateProfileApiMutation,
  useProfileApiQuery,
} from '../../../shared/api'
import type { ProfileId } from '../../../shared/model'

export class ProfileScopeError extends Error {
  constructor(message: string) {
    super(message)
    this.name = 'ProfileScopeError'
  }
}

export type ProfileActorScope = {
  readonly actorId: ProfileId | null
  readonly error: ProfileScopeError | null
  readonly isReady: boolean
}

function resolveProfileActorScope(profileId: string | null | undefined, actorId: ProfileId | null): ProfileActorScope {
  if (!profileId) {
    return {
      actorId,
      error: new ProfileScopeError('Profile id is required for profile-scoped requests.'),
      isReady: false,
    }
  }

  if (!actorId) {
    return {
      actorId,
      error: new ProfileScopeError(`Profile "${profileId}" requires an active actor id.`),
      isReady: false,
    }
  }

  if (actorId !== profileId) {
    return {
      actorId,
      error: new ProfileScopeError(`Active actor "${actorId}" cannot access profile "${profileId}".`),
      isReady: false,
    }
  }

  return {
    actorId,
    error: null,
    isReady: true,
  }
}

function requireScopedActorId(profileId: string | null | undefined, actorId: ProfileId | null): ProfileId {
  const scope = resolveProfileActorScope(profileId, actorId)

  if (!scope.isReady || !scope.actorId) {
    throw scope.error ?? new ProfileScopeError('Profile actor scope is not ready.')
  }

  return scope.actorId
}

export function useProfileActorScope(profileId: string | null | undefined): ProfileActorScope {
  const actorId = useAppStore((state) => state.profileId)

  return resolveProfileActorScope(profileId, actorId)
}

export type UseProfileQueryOptions = Omit<UseProfileApiQueryOptions, 'request'>

export function useProfileQuery(
  profileId: ProfileId | string,
  options: UseProfileQueryOptions = {},
): UseQueryResult<Profile, Error> {
  const scope = useProfileActorScope(profileId)
  const { enabled, ...query } = options

  return useProfileApiQuery(profileId, {
    ...query,
    enabled: (enabled ?? true) && scope.isReady,
    request: scope.actorId ? { actorId: scope.actorId } : undefined,
  }) as UseQueryResult<Profile, Error>
}

export function useCurrentProfileQuery(options: UseProfileQueryOptions = {}): UseQueryResult<Profile, Error> {
  const profileId = useAppStore((state) => state.profileId)

  return useProfileQuery(profileId ?? '', {
    ...options,
    enabled: (options.enabled ?? true) && Boolean(profileId),
  })
}

export function useCreateProfileMutation(
  options: UseCreateProfileApiMutationOptions = {},
): UseMutationResult<Profile, Error, CreateProfileDto, unknown> {
  return useCreateProfileApiMutation(options) as UseMutationResult<Profile, Error, CreateProfileDto, unknown>
}

export type UseUpdateProfileMutationOptions = Omit<UseUpdateProfileApiMutationOptions, 'request'>

export function useUpdateProfileMutation(
  profileId: ProfileId | string,
  options: UseUpdateProfileMutationOptions = {},
): UseMutationResult<Profile, Error, UpdateProfileDto, unknown> {
  const actorId = useAppStore((state) => state.profileId)
  const queryClient = useQueryClient()
  const { onSuccess, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: (payload) =>
      updateProfile(profileId, payload, {
        actorId: requireScopedActorId(profileId, actorId),
      }),
    onSuccess: (profile, variables, onMutateResult, context) => {
      queryClient.setQueryData(profileQueryKey(profile.id), profile)
      onSuccess?.(profile, variables, onMutateResult, context)
    },
  }) as UseMutationResult<Profile, Error, UpdateProfileDto, unknown>
}

export function useCurrentProfileUpdateMutation(
  options: UseUpdateProfileMutationOptions = {},
): UseMutationResult<Profile, Error, UpdateProfileDto, unknown> {
  const profileId = useAppStore((state) => state.profileId)

  return useUpdateProfileMutation(profileId ?? '', options)
}
