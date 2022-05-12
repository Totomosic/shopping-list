import Auth from "@/store/defs/auth"
import { IJwtToken } from "@/utils/types"
import { Router } from "vue-router"
import { GlobalStore } from ".."

export class AuthWrapper {
  private store: GlobalStore

  public constructor(store: GlobalStore) {
    this.store = store
  }

  public get loginFormVisible(): boolean {
    return this.store.state.auth.loginFormVisible
  }

  public get user(): IJwtToken | null {
    return this.store.state.auth.user
  }

  public async waitForLogin(): Promise<boolean> {
    const success = await this.store.dispatch(Auth.Actions.WaitForLogin)
    if (success) {
      await this.loadUser()
    }
    return success
  }

  public notifyLoginComplete(success: boolean) {
    this.store.dispatch(Auth.Actions.LoginComplete, success)
  }

  public async loadUser() {
    await this.store.dispatch(Auth.Actions.LoadUser, this.store)
  }

  public clearUser() {
    this.store.dispatch(Auth.Actions.ClearUser)
  }

  public async loadAdminUserOrGoHome(router: Router) {
    await this.loadUser()
    if (!this.user || !this.user.is_admin) {
      router.push({ name: "Home" })
    }
  }

  public async logoutAndGoHome(router: Router) {
    this.clearUser()
    router.push({ name: "Home" })
  }
}
