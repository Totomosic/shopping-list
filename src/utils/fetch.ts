import { GlobalStore } from "@/store"
import { getJwtHeaders } from "./jwt"
import { Dictionary } from "./types"

export interface IFetchOptions {
  method: "GET" | "POST" | "PATCH" | "DELETE"
  headers: Dictionary<string>
  body: any
}

const DEFAULT_FETCH_OPTIONS: IFetchOptions = {
  method: "GET",
  headers: {},
  body: null,
}

export async function fetchJson<T>(endpoint: string, options?: Partial<IFetchOptions>): Promise<T | null> {
  try {
    const opts = { ...DEFAULT_FETCH_OPTIONS, ...options }
    const response = await fetch(endpoint, {
      method: opts.method,
      headers: { ...opts.headers, "Content-Type": "application/json" },
      body: opts.body ? JSON.stringify(opts.body) : undefined,
    })
    const result = await response.json()
    return result as T | null
  } catch (err) {
    return null
  }
}

export async function fetchJsonAuthenticated<T>(
  endpoint: string,
  store: GlobalStore,
  options?: Partial<IFetchOptions>
): Promise<T | null> {
  return fetchJson<T>(endpoint, {
    ...options,
    headers: {
      ...options?.headers,
      ...(await getJwtHeaders(store)),
    },
  })
}
