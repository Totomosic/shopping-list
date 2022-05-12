import { createStore, useStore as baseUseStore } from "vuex"
import { InjectionKey } from "vue"
import { EventBus } from "./bus"
import createAuthModule, { AuthState } from "./modules/auth"
import { AugmentedStore } from "./register"
import { AuthWrapper } from "./wrappers/auth"

export type State = {
  auth: AuthState
}

export type GlobalStore = AugmentedStore<State> & {
  wrappers: {
    auth: AuthWrapper
  }
}

// https://next.vuex.vuejs.org/guide/typescript-support.html#typing-usestore-composition-function
export const key: InjectionKey<GlobalStore> = Symbol("Store injection key")

const bus = new EventBus()

const store = createStore({
  modules: {
    auth: createAuthModule(bus) as any,
  },
}) as GlobalStore
store.bus = bus
store.wrappers = {
  auth: new AuthWrapper(store),
}

export function useStore() {
  return baseUseStore(key) as GlobalStore
}

export default store
