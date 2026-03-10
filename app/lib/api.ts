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
  const response = await fetch('/api/hello')
  return handleResponse<HelloResponse>(response)
}

export interface GreetResponse {
  message: string
}

export async function greet(name: string): Promise<GreetResponse> {
  const response = await fetch(`/api/greet/${encodeURIComponent(name)}`)
  return handleResponse<GreetResponse>(response)
}

export interface SearchResponse {
  message: string
  data?: Record<string, unknown>
}

export async function search(query: string): Promise<SearchResponse> {
  const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`)
  return handleResponse<SearchResponse>(response)
}
