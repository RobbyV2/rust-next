export function resolveApiBase(
  url: string | undefined,
  hasWindow: boolean,
  host?: string,
  port?: string
): string {
  if (url) return `${url.replace(/\/+$/, '')}/api`
  if (hasWindow) return '/api'
  return `http://${host || '127.0.0.1'}:${port || '3000'}/api`
}

const API = resolveApiBase(
  process.env.NEXT_PUBLIC_API_URL,
  typeof window !== 'undefined',
  process.env.HOST,
  process.env.SERVER_PORT
)

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public data?: unknown
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: response.statusText }))
    throw new ApiError(error.error || response.statusText, response.status, error)
  }
  return response.json()
}

export interface HelloResponse {
  message: string
  data?: Record<string, unknown>
}

export async function hello(): Promise<HelloResponse> {
  const response = await fetch(`${API}/hello`)
  return handleResponse<HelloResponse>(response)
}

export interface GreetResponse {
  message: string
}

export async function greet(name: string): Promise<GreetResponse> {
  const response = await fetch(`${API}/greet/${encodeURIComponent(name)}`)
  return handleResponse<GreetResponse>(response)
}

export interface SearchResponse {
  message: string
  data?: Record<string, unknown>
}

export async function search(query: string): Promise<SearchResponse> {
  const response = await fetch(`${API}/search?q=${encodeURIComponent(query)}`)
  return handleResponse<SearchResponse>(response)
}
