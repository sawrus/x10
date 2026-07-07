import { ApiError, type ApiValue } from './types'

const DEFAULT_BASE_URL = 'http://127.0.0.1:3000'
const jsonContentType = 'application/json'

type JsonBody = ApiValue | Record<string, unknown>

export type ApiRequestOptions = Omit<RequestInit, 'body' | 'headers'> & {
  actorId?: string
  body?: BodyInit | JsonBody
  headers?: HeadersInit
}

function resolveBaseUrl(): string {
  return import.meta.env.VITE_API_BASE_URL || import.meta.env.VITE_API_URL || DEFAULT_BASE_URL
}

function resolveUrl(path: string): string {
  if (/^https?:\/\//.test(path)) {
    return path
  }

  return new URL(path, resolveBaseUrl()).toString()
}

function isJsonBody(body: ApiRequestOptions['body']): body is JsonBody {
  if (body == null || typeof body === 'string') {
    return false
  }

  if (body instanceof ArrayBuffer || ArrayBuffer.isView(body)) {
    return false
  }

  if (body instanceof Blob || body instanceof FormData || body instanceof URLSearchParams) {
    return false
  }

  return typeof ReadableStream !== 'function' || !(body instanceof ReadableStream)
}

async function parseResponse(response: Response): Promise<ApiValue | null> {
  if (response.status === 204) {
    return null
  }

  const contentType = response.headers.get('content-type') || ''
  if (!contentType.includes(jsonContentType)) {
    const text = await response.text()
    return text || null
  }

  return (await response.json()) as ApiValue
}

async function request<TResponse>(path: string, options: ApiRequestOptions = {}): Promise<TResponse> {
  const headers = new Headers(options.headers)
  const url = resolveUrl(path)
  const body = isJsonBody(options.body) ? JSON.stringify(options.body) : options.body

  headers.set('accept', jsonContentType)
  if (options.actorId) {
    headers.set('x-actor-id', options.actorId)
  }
  if (isJsonBody(options.body) && !headers.has('content-type')) {
    headers.set('content-type', jsonContentType)
  }

  const response = await fetch(url, {
    ...options,
    body,
    credentials: options.credentials ?? 'same-origin',
    headers,
  })

  const payload = await parseResponse(response)
  if (!response.ok) {
    throw new ApiError({
      status: response.status,
      statusText: response.statusText,
      url,
      data: payload,
    })
  }

  return payload as TResponse
}

export const apiClient = {
  delete: <TResponse>(path: string, options?: ApiRequestOptions) =>
    request<TResponse>(path, { ...options, method: 'DELETE' }),
  get: <TResponse>(path: string, options?: ApiRequestOptions) => request<TResponse>(path, { ...options, method: 'GET' }),
  patch: <TResponse>(path: string, options?: ApiRequestOptions) =>
    request<TResponse>(path, { ...options, method: 'PATCH' }),
  post: <TResponse>(path: string, options?: ApiRequestOptions) => request<TResponse>(path, { ...options, method: 'POST' }),
  put: <TResponse>(path: string, options?: ApiRequestOptions) => request<TResponse>(path, { ...options, method: 'PUT' }),
}
