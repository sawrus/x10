import { queryOptions, useQuery, type UseQueryOptions } from '@tanstack/react-query'

import { apiClient, type ApiRequestOptions } from './client'

export type HealthResponse = {
  status: string
}

export const healthQueryKey = ['health'] as const

export function getHealth(options?: ApiRequestOptions) {
  return apiClient.get<HealthResponse>('/health', options)
}

export function getHealthQueryOptions(options?: ApiRequestOptions) {
  return queryOptions({
    queryKey: healthQueryKey,
    queryFn: ({ signal }) => getHealth({ ...options, signal: options?.signal ?? signal }),
  })
}

export type UseHealthQueryOptions = Omit<
  UseQueryOptions<HealthResponse, Error, HealthResponse, typeof healthQueryKey>,
  'queryKey' | 'queryFn'
> & {
  request?: ApiRequestOptions
}

export function useHealthQuery(options: UseHealthQueryOptions = {}) {
  const { request, ...query } = options

  return useQuery({
    ...getHealthQueryOptions(request),
    ...query,
  })
}
