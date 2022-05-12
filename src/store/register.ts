import { EventBus } from "@/store/bus"
import { createStore, Store, StoreOptions } from "vuex"

/**
 * The purpose of this function is to generate a pseudo namespace object that can be used to make
 * Vuex's commit(...) and dispatch(...) store methods slightly more type safe by using enum method names
 * instead of plain strings.
 * This function accepts a typescript namespace containing typescript enum definitions and returns a new object
 * with the same structure but the enum values are replaced with unique strings that can be used directly in
 * commit(...) and dispatch(...)
 * eg. Consider the namespace definition
 * ```typescript
 * namespace Example {
 *    export enum Mutations {
 *      SetValue,
 *    }
 * }
 * const NewNamespace = registerNamespace("Example", Example)
 * ```
 * Is equivalent to:
 * ```typescript
 * const NewNamespace = {
 *    Mutations: {
 *      SetValue: "Example.SetValue",
 *    }
 * }
 * ```
 * @param namespace A unique string representing the namespace of the scope
 * @param scope typically a typescript namespace containing a set of Vuex Enums (Mutations, Actions, Getters)
 * @returns a new generated namespace object where the enum values have been converted to unique string values
 */
export function registerNamespace<T>(namespace: string, scope: T): { [K in keyof T]: { [K2 in keyof T[K]]: string } } {
  const result: any = {}
  for (const key of Object.keys(scope) as (keyof T)[]) {
    const newEnum: any = {}
    for (const val of Object.keys(scope[key])) {
      newEnum[val] = `${namespace}.${(scope as any)[key][(scope as any)[key][val]]}`
    }
    result[key] = newEnum
  }
  return result
}

export type AugmentedStore<T> = Store<T> & { bus: EventBus }
export type ExactStoreNamespace<TGetters, TActions, TMutations, TEvents> = {
  Getters: TGetters
  Actions: TActions
  Mutations: TMutations
  Events: TEvents
}

export type MapType<T> = { [K in keyof T]: string }
export type MappedStoreNamespace<TGetters, TActions, TMutations, TEvents> = {
  Getters: MapType<TGetters>
  Actions: MapType<TActions>
  Mutations: MapType<TMutations>
  Events: MapType<TEvents>
}

export type StoreNamespace<TGetters, TActions, TMutations, TEvents> = Partial<
  ExactStoreNamespace<TGetters, TActions, TMutations, TEvents>
>
export type StoreDefinitionCallback<T, TGetters, TActions, TMutations, TEvents> = (
  defs: MappedStoreNamespace<TGetters, TActions, TMutations, TEvents>,
  bus: EventBus
) => StoreOptions<T>
export type ScopedStore<T, TGetters, TActions, TMutations, TEvents> = AugmentedStore<T> &
  MappedStoreNamespace<TGetters, TActions, TMutations, TEvents>

export function scopedStore<T, TGetters, TActions, TMutations, TEvents>(
  name: string,
  namespace: StoreNamespace<TGetters, TActions, TMutations, TEvents>,
  storeCreate: StoreDefinitionCallback<T, TGetters, TActions, TMutations, TEvents>
): ScopedStore<T, TGetters, TActions, TMutations, TEvents> {
  const bus = new EventBus()
  const transformedNamespace = registerNamespace(name, namespace)
  const store = createStore(storeCreate(transformedNamespace as any, bus)) as ScopedStore<
    T,
    TGetters,
    TActions,
    TMutations,
    TEvents
  >
  store.bus = bus
  store.Mutations = transformedNamespace.Mutations! as any
  store.Actions = transformedNamespace.Actions! as any
  store.Events = transformedNamespace.Events! as any
  store.Getters = transformedNamespace.Getters! as any
  return store
}
