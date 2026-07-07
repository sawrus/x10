import { apiClient, type ApiRequestOptions } from './client'

export type HealthResponse = {
  status: string
}

export function getHealth(options?: ApiRequestOptions) {
  return apiClient.get<HealthResponse>('/health', options)
}
