import { isNullOrUndefined, stableSort } from "./utils"

export type Dictionary<T> = { [key: string]: T }

export interface IJwtToken {
  token_type: "access" | "refresh"
  exp: number
  user_id: number
  display_name: string
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

export interface IUser {
  id: number
  display_name: string
  is_admin: boolean
}

export interface INewUser {
  display_name: string
  username: string
  password: string
  is_admin: boolean
}

export const EVENT_EMITTER_PRIORITY_DEFAULT = 0
export const EVENT_EMITTER_PRIORITY_MAX = 1000
export const EVENT_EMITTER_PRIORITY_MIN = -1000

interface IEventListener<T extends any[]> {
  listener: (...args: T) => void | Promise<void>
  priority: number
}

export type UnsubscribeEventListener = () => void

export class EventEmitter<T extends any[]> {
  private listeners: IEventListener<T>[] = []
  private sendEvents: boolean = true
  private lastTriggerDataValid: boolean = false
  private lastTriggerData: T | null = null
  private hasPriorities: boolean = false

  public get hasListeners(): boolean {
    return this.listeners.length > 0
  }

  public suspendEvents(): void {
    this.sendEvents = false
  }

  public resumeEvents(): void {
    this.sendEvents = true
    if (this.lastTriggerDataValid) {
      this.lastTriggerDataValid = false
      this.triggerInternal(...this.lastTriggerData!)
    }
  }

  public addEventListener(listener: (...args: T) => void | Promise<void>, priority?: number): UnsubscribeEventListener {
    const listenerData: IEventListener<T> = {
      listener,
      priority: priority ?? EVENT_EMITTER_PRIORITY_DEFAULT,
    }
    this.listeners.push(listenerData)
    if (!isNullOrUndefined(priority)) {
      this.hasPriorities = true
    }
    return () => {
      this.listeners.splice(this.listeners.indexOf(listenerData), 1)
    }
  }

  public async trigger(...args: T): Promise<void> {
    if (this.sendEvents) {
      return this.triggerInternal(...args)
    }
    this.lastTriggerData = args
    this.lastTriggerDataValid = true
    // TODO: Maybe return promise here?
  }

  private async triggerInternal(...args: T): Promise<void> {
    if (this.hasPriorities) {
      for (const listener of this.getListeners()) {
        // If we are using priorities then events have to be completed in the correct order
        /* eslint-disable-next-line */
        await listener.listener(...args)
      }
    } else {
      const promises: (void | Promise<void>)[] = []
      for (const listener of this.getListeners()) {
        promises.push(listener.listener(...args))
      }
      await Promise.all(promises)
    }
  }

  private getListeners() {
    if (this.hasPriorities) {
      return stableSort(this.listeners, (a, b) => b.priority - a.priority)
    }
    return this.listeners
  }
}
