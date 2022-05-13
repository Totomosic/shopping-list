import { GlobalStore } from "@/store"
import { fetchJsonAuthenticated, IFetchOptions } from "./fetch"
import { GenericResponse, INewUser, IUser } from "./types"
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

  public async createNewUser(user: INewUser): Promise<IUser | null> {
    const createdUser = await this.fetch<IUser>("/users", {
      method: "POST",
      body: user,
    })
    return createdUser?.data ?? null
  }

  public async deleteUser(userId: number): Promise<IUser | null> {
    const deletedUser = await this.fetch<IUser>(`/users/${userId}`, {
      method: "DELETE",
    })
    return deletedUser?.data ?? null
  }

  private async fetch<T>(endpoint: string, options?: Partial<IFetchOptions>): Promise<GenericResponse<T> | null> {
    return fetchJsonAuthenticated<GenericResponse<T>>(`${API_BASE}${endpoint}`, this.store, options)
  }
}
