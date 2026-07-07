export type ApiPrimitive = boolean | number | string | null
export type ApiValue = ApiPrimitive | ApiValue[] | { [key: string]: ApiValue | undefined }

export type ApiErrorDetails = {
  code?: string
  details?: ApiValue
  message?: string
}

export class ApiError extends Error {
  readonly status: number
  readonly statusText: string
  readonly url: string
  readonly data: ApiValue | null

  constructor(params: { status: number; statusText: string; url: string; data: ApiValue | null }) {
    super(params.data && typeof params.data === 'object' && 'message' in params.data && typeof params.data.message === 'string'
      ? params.data.message
      : `API request failed with status ${params.status}`)
    this.name = 'ApiError'
    this.status = params.status
    this.statusText = params.statusText
    this.url = params.url
    this.data = params.data
  }

  get details(): ApiErrorDetails | null {
    if (!this.data || typeof this.data !== 'object' || Array.isArray(this.data)) {
      return null
    }

    if ('error' in this.data && this.data.error && typeof this.data.error === 'object' && !Array.isArray(this.data.error)) {
      return this.data.error as ApiErrorDetails
    }

    return this.data as ApiErrorDetails
  }
}

export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError
}
