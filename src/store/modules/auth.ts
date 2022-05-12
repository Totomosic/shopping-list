import Auth from "@/store/defs/auth"
import { clearJwtTokens, getRefreshTokenOrLogin, isLoggedIn } from "@/utils/jwt"
import { IJwtToken } from "@/utils/types"
import { ActionContext } from "vuex"
import { GlobalStore } from ".."
import { EventBus } from "../bus"

/* eslint-disable-next-line */
export interface AuthState {
  loginFormVisible: boolean
  loginPromise: Promise<boolean> | null
  loginPromiseResolve: ((result: boolean) => void) | null
  user: IJwtToken | null
}

export default function createAuthModule(bus: EventBus) {
  const stateObject: AuthState = {
    loginFormVisible: false,
    loginPromise: null,
    loginPromiseResolve: null,
    user: null,
  }

  const gettersObject = {}

  const actionsObject = {
    async [Auth.Actions.WaitForLogin]({ commit, state }: ActionContext<AuthState, AuthState>) {
      if (!isLoggedIn()) {
        commit(Auth.Mutations.SetLoginFormVisibility, true)
        if (!state.loginPromise) {
          state.loginPromise = new Promise<boolean>((resolve) => {
            state.loginPromiseResolve = resolve
          }).then((result) => {
            state.loginPromise = null
            state.loginPromiseResolve = null
            return result
          })
        }
        return state.loginPromise
      }
      return true
    },
    [Auth.Actions.LoginComplete]({ commit, state }: ActionContext<AuthState, AuthState>, success: boolean) {
      if (state.loginPromiseResolve) {
        state.loginPromiseResolve(success)
      }
      commit(Auth.Mutations.SetLoginFormVisibility, false)
    },
    async [Auth.Actions.LoadUser]({ state }: ActionContext<AuthState, AuthState>, store: GlobalStore) {
      const token = await getRefreshTokenOrLogin(store)
      if (token) {
        state.user = token
      } else {
        state.user = null
      }
    },
    [Auth.Actions.ClearUser]({ state }: ActionContext<AuthState, AuthState>) {
      clearJwtTokens()
      state.user = null
    },
  }

  const mutationsObject = {
    [Auth.Mutations.SetLoginFormVisibility](state: AuthState, visible: boolean) {
      state.loginFormVisible = visible
    },
  }

  return {
    state: stateObject,
    getters: gettersObject,
    actions: actionsObject,
    mutations: mutationsObject,
  }
}
