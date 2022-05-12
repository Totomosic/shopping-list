import { GlobalStore } from "@/store"
import jwt_decode from "jwt-decode"
import { fetchJson } from "./fetch"
import { Dictionary, GenericResponse, IJwtToken } from "./types"
import { API_BASE, isSuccessResponse } from "./utils"

function getJwtLocalStoragePrefix(endpoint: string): string {
  const transformedEndpoint = endpoint.replaceAll(":", "_").replace("/", "_")
  return `shopping_jwt_${transformedEndpoint}`
}

const JWT_ACCESS_STORAGE_KEY = `${getJwtLocalStoragePrefix(API_BASE)}_access`
const JWT_REFRESH_STORAGE_KEY = `${getJwtLocalStoragePrefix(API_BASE)}_refresh`

export function decodeJwt(jwt: string): IJwtToken | null {
  try {
    const token = jwt_decode(jwt) as IJwtToken
    return token
  } catch (err) {
    console.error("JWT Decode Error:", err)
    return null
  }
}

export function isExpired(token: IJwtToken, leniancy: number = 1000): boolean {
  return token.exp + leniancy < Date.now()
}

export function saveRefreshToken(token: string): void {
  localStorage.setItem(JWT_REFRESH_STORAGE_KEY, token)
}

export function saveAccessToken(token: string): void {
  localStorage.setItem(JWT_ACCESS_STORAGE_KEY, token)
}

export function clearJwtTokens(): void {
  localStorage.removeItem(JWT_ACCESS_STORAGE_KEY)
  localStorage.removeItem(JWT_REFRESH_STORAGE_KEY)
}

function getJwtHeadersFromAccessToken(token: string): Dictionary<string> {
  return {
    Authorization: `Bearer ${token}`,
  }
}

export async function getJwtHeaders(store: GlobalStore): Promise<Dictionary<string> | null> {
  const accessToken = localStorage.getItem(JWT_ACCESS_STORAGE_KEY)
  if (accessToken) {
    const parsedAccessToken = decodeJwt(accessToken)
    if (parsedAccessToken && !isExpired(parsedAccessToken)) {
      return getJwtHeadersFromAccessToken(accessToken)
    }
  }

  const refreshToken = await getRefreshTokenOrLogin(store)
  if (refreshToken) {
    const response = await fetchJson<GenericResponse<{ token: string }>>(`${API_BASE}/core/auth/refresh`, {
      method: "POST",
      body: {
        token: localStorage.getItem(JWT_REFRESH_STORAGE_KEY),
      },
    })
    if (isSuccessResponse(response)) {
      return getJwtHeadersFromAccessToken(response.data.token)
    }
  }
  return null
}

export function getRefreshToken(): IJwtToken | null {
  const refreshToken = localStorage.getItem(JWT_REFRESH_STORAGE_KEY)
  if (refreshToken) {
    const parsedRefreshToken = decodeJwt(refreshToken)
    if (!!parsedRefreshToken && !isExpired(parsedRefreshToken)) {
      return parsedRefreshToken
    }
  }
  return null
}

export function isLoggedIn(): boolean {
  return !!getRefreshToken()
}

export async function getRefreshTokenOrLogin(store: GlobalStore): Promise<IJwtToken | null> {
  const token = getRefreshToken()
  if (token) {
    return token
  }
  await store.wrappers.auth.waitForLogin()
  return getRefreshToken()
}
