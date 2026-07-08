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
    let errorMessage = `API request failed with status ${params.status}`

    if (typeof params.data === 'string' && params.data.trim()) {
      errorMessage = params.data.trim()
    } else if (params.data && typeof params.data === 'object' && !Array.isArray(params.data)) {
      if ('message' in params.data && typeof params.data.message === 'string') {
        errorMessage = params.data.message
      } else if (
        'error' in params.data &&
        params.data.error &&
        typeof params.data.error === 'object' &&
        !Array.isArray(params.data.error) &&
        'message' in params.data.error &&
        typeof params.data.error.message === 'string'
      ) {
        errorMessage = params.data.error.message
      }
    }

    super(errorMessage)
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
