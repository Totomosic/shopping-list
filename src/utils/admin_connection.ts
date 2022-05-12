import { GlobalStore } from "@/store"
import { fetchJsonAuthenticated, IFetchOptions } from "./fetch"
import { GenericResponse, IUser } from "./types"
import { API_BASE, isSuccessResponse } from "./utils"

export class AdminDataConnection {
  private store: GlobalStore

  public constructor(store: GlobalStore) {
    this.store = store
  }

  public async getAllUsers(): Promise<IUser[]> {
    const users = await this.fetch<IUser[]>("/users")
    if (isSuccessResponse(users)) {
      return users.data
    }
    return []
  }

  private async fetch<T>(endpoint: string, options?: Partial<IFetchOptions>): Promise<GenericResponse<T> | null> {
    return fetchJsonAuthenticated<GenericResponse<T>>(`${API_BASE}${endpoint}`, this.store, options)
  }
}
