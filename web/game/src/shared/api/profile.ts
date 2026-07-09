import { queryOptions, useMutation, useQuery, useQueryClient, type UseMutationOptions, type UseQueryOptions } from '@tanstack/react-query'

import type { CreateProfileDto, Profile, ProfilePhotoSummary, UpdateProfileDto } from '../../entities/profile/model/types'
import type { ApiError } from './types'
import { apiClient, resolveApiUrl, type ApiRequestOptions } from './client'

export type ProfileRequestOptions = Omit<ApiRequestOptions, 'actorId'> & {
  actorId?: string
}

export const profilesQueryKey = ['profiles'] as const
export const profileQueryKey = (profileId: string) => [...profilesQueryKey, 'detail', profileId] as const
export const profilePhotosQueryKey = (profileId: string) => [...profileQueryKey(profileId), 'photos'] as const

export function listProfiles(options?: ApiRequestOptions) {
  return apiClient.get<Profile[]>('/api/v2/profiles', options)
}

function requireProfileActorId(profileId: string, options?: ProfileRequestOptions): ApiRequestOptions {
  const actorId = options?.actorId?.trim()

  if (!actorId) {
    throw new Error(`Profile request for "${profileId}" requires X-Actor-Id.`)
  }

  return {
    ...options,
    actorId,
  }
}

export function createProfile(payload: CreateProfileDto, options?: ApiRequestOptions) {
  return apiClient.post<Profile>('/api/v2/profiles', {
    ...options,
    body: payload,
  })
}

export function getProfile(profileId: string, options?: ProfileRequestOptions) {
  return apiClient.get<Profile>(`/api/v2/profiles/${profileId}`, requireProfileActorId(profileId, options))
}

export function updateProfile(profileId: string, payload: UpdateProfileDto, options?: ProfileRequestOptions) {
  return apiClient.patch<Profile>(`/api/v2/profiles/${profileId}`, {
    ...requireProfileActorId(profileId, options),
    body: payload,
  })
}

export function listProfilePhotos(profileId: string, options?: ProfileRequestOptions) {
  return apiClient.get<ProfilePhotoSummary[]>(`/api/v2/profiles/${profileId}/photos`, requireProfileActorId(profileId, options))
}

export function uploadProfilePhoto(profileId: string, file: File, options?: ProfileRequestOptions) {
  const body = new FormData()
  body.set('file', file)

  return apiClient.post<ProfilePhotoSummary>(`/api/v2/profiles/${profileId}/photos`, {
    ...requireProfileActorId(profileId, options),
    body,
  })
}

export function deleteProfilePhoto(photoId: string, profileId: string, options?: ProfileRequestOptions) {
  return apiClient.delete<null>(`/api/v2/photos/${photoId}`, requireProfileActorId(profileId, options))
}

export function selectProfilePhoto(profileId: string, photoId: string, options?: ProfileRequestOptions) {
  return apiClient.post<Profile>(`/api/v2/profiles/${profileId}/photos/${photoId}/select`, requireProfileActorId(profileId, options))
}

export async function getProfilePhotoBlob(photoId: string, profileId: string, options?: ProfileRequestOptions): Promise<Blob> {
  const request = requireProfileActorId(profileId, options)
  const response = await fetch(resolveApiUrl(`/api/v2/photos/${photoId}`), {
    credentials: request.credentials ?? 'same-origin',
    headers: {
      accept: '*/*',
      ...(request.actorId ? { 'x-actor-id': request.actorId } : {}),
      ...(request.headers ?? {}),
    },
    method: 'GET',
    signal: request.signal,
  })

  if (!response.ok) {
    throw new Error(`Unable to load photo asset ${photoId}: ${response.status}`)
  }

  return response.blob()
}

export function getProfileQueryOptions(profileId: string, options?: ProfileRequestOptions) {
  return queryOptions<Profile, ApiError, Profile, ReturnType<typeof profileQueryKey>>({
    queryKey: profileQueryKey(profileId),
    queryFn: ({ signal }) =>
      getProfile(profileId, {
        ...options,
        signal: options?.signal ?? signal,
      }),
  })
}

export function getProfilesQueryOptions(options?: ApiRequestOptions) {
  return queryOptions<Profile[], ApiError, Profile[], typeof profilesQueryKey>({
    queryKey: profilesQueryKey,
    queryFn: ({ signal }) =>
      listProfiles({
        ...options,
        signal: options?.signal ?? signal,
      }),
  })
}

export function getProfilePhotosQueryOptions(profileId: string, options?: ProfileRequestOptions) {
  return queryOptions<ProfilePhotoSummary[], ApiError, ProfilePhotoSummary[], ReturnType<typeof profilePhotosQueryKey>>({
    queryKey: profilePhotosQueryKey(profileId),
    queryFn: ({ signal }) =>
      listProfilePhotos(profileId, {
        ...options,
        signal: options?.signal ?? signal,
      }),
  })
}

export type UseProfileApiQueryOptions = Omit<
  UseQueryOptions<Profile, ApiError, Profile, ReturnType<typeof profileQueryKey>>,
  'queryKey' | 'queryFn'
> & {
  request?: ProfileRequestOptions
}

export function useProfileApiQuery(profileId: string, options: UseProfileApiQueryOptions = {}) {
  const { request, ...query } = options

  return useQuery<Profile, ApiError, Profile, ReturnType<typeof profileQueryKey>>({
    ...getProfileQueryOptions(profileId, request),
    ...query,
  })
}

export type UseProfilesApiQueryOptions = Omit<
  UseQueryOptions<Profile[], ApiError, Profile[], typeof profilesQueryKey>,
  'queryKey' | 'queryFn'
> & {
  request?: ApiRequestOptions
}

export function useProfilesApiQuery(options: UseProfilesApiQueryOptions = {}) {
  const { request, ...query } = options

  return useQuery<Profile[], ApiError, Profile[], typeof profilesQueryKey>({
    ...getProfilesQueryOptions(request),
    ...query,
  })
}

export type UseProfilePhotosApiQueryOptions = Omit<
  UseQueryOptions<ProfilePhotoSummary[], ApiError, ProfilePhotoSummary[], ReturnType<typeof profilePhotosQueryKey>>,
  'queryKey' | 'queryFn'
> & {
  request?: ProfileRequestOptions
}

export function useProfilePhotosApiQuery(profileId: string, options: UseProfilePhotosApiQueryOptions = {}) {
  const { request, ...query } = options

  return useQuery<ProfilePhotoSummary[], ApiError, ProfilePhotoSummary[], ReturnType<typeof profilePhotosQueryKey>>({
    ...getProfilePhotosQueryOptions(profileId, request),
    ...query,
  })
}

export type UseCreateProfileApiMutationOptions = Omit<UseMutationOptions<Profile, ApiError, CreateProfileDto>, 'mutationFn'> & {
  request?: ApiRequestOptions
}

export function useCreateProfileApiMutation(options: UseCreateProfileApiMutationOptions = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, request, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: (payload) => createProfile(payload, request),
    onSuccess: async (profile, variables, onMutateResult, context) => {
      queryClient.setQueryData(profileQueryKey(profile.id), profile)
      await queryClient.invalidateQueries({ queryKey: profilesQueryKey })
      onSuccess?.(profile, variables, onMutateResult, context)
    },
  })
}

export type UseUpdateProfileApiMutationOptions = Omit<UseMutationOptions<Profile, ApiError, UpdateProfileDto>, 'mutationFn'> & {
  request?: ProfileRequestOptions
}

export function useUpdateProfileApiMutation(profileId: string, options: UseUpdateProfileApiMutationOptions = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, request, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: (payload) => updateProfile(profileId, payload, request),
    onSuccess: (profile, variables, onMutateResult, context) => {
      queryClient.setQueryData(profileQueryKey(profile.id), profile)
      onSuccess?.(profile, variables, onMutateResult, context)
    },
  })
}

export type UseUploadProfilePhotoApiMutationOptions = Omit<
  UseMutationOptions<ProfilePhotoSummary, ApiError, File>,
  'mutationFn'
> & {
  request?: ProfileRequestOptions
}

export function useUploadProfilePhotoApiMutation(profileId: string, options: UseUploadProfilePhotoApiMutationOptions = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, request, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: (file) => uploadProfilePhoto(profileId, file, request),
    onSuccess: async (photo, variables, onMutateResult, context) => {
      await queryClient.invalidateQueries({ queryKey: profilePhotosQueryKey(profileId) })
      onSuccess?.(photo, variables, onMutateResult, context)
    },
  })
}

export type UseDeleteProfilePhotoApiMutationOptions = Omit<
  UseMutationOptions<null, ApiError, { photoId: string }>,
  'mutationFn'
> & {
  request?: ProfileRequestOptions
}

export function useDeleteProfilePhotoApiMutation(profileId: string, options: UseDeleteProfilePhotoApiMutationOptions = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, request, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: ({ photoId }) => deleteProfilePhoto(photoId, profileId, request),
    onSuccess: async (response, variables, onMutateResult, context) => {
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: profilePhotosQueryKey(profileId) }),
        queryClient.invalidateQueries({ queryKey: profileQueryKey(profileId) }),
      ])
      onSuccess?.(response, variables, onMutateResult, context)
    },
  })
}

export type UseSelectProfilePhotoApiMutationOptions = Omit<
  UseMutationOptions<Profile, ApiError, { photoId: string }>,
  'mutationFn'
> & {
  request?: ProfileRequestOptions
}

export function useSelectProfilePhotoApiMutation(profileId: string, options: UseSelectProfilePhotoApiMutationOptions = {}) {
  const queryClient = useQueryClient()
  const { onSuccess, request, ...mutation } = options

  return useMutation({
    ...mutation,
    mutationFn: ({ photoId }) => selectProfilePhoto(profileId, photoId, request),
    onSuccess: async (profile, variables, onMutateResult, context) => {
      queryClient.setQueryData(profileQueryKey(profile.id), profile)
      await queryClient.invalidateQueries({ queryKey: profilePhotosQueryKey(profileId) })
      onSuccess?.(profile, variables, onMutateResult, context)
    },
  })
}
