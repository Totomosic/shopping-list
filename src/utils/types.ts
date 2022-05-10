export type Dictionary<T> = { [key: string]: T }

export interface IJwtToken {
  token_type: "access" | "refresh"
  exp: number
  user_id: number
  is_admin: boolean
}

export type GenericResponse<T> = {
  success: boolean
  error: string | null
  data: T | null
}

export type SuccessResponse<T> = {
  success: true
  error: null
  data: T
}

export interface ILoginResponse {
  refresh_token: string
  access_token: string
}
