type Callback = (...args: any[]) => void | Promise<void>

export class EventBus {
  private listenerMap: Map<string, Callback[]> = new Map()

  public addEventListener(evt: string, listener: Callback): () => void {
    const callbacks = this.listenerMap.get(evt)
    if (callbacks) {
      callbacks.push(listener)
    } else {
      this.listenerMap.set(evt, [listener])
    }
    return () => {
      const listeners = this.listenerMap.get(evt)
      if (listeners) {
        const index = listeners.indexOf(listener)
        if (index >= 0) {
          listeners.splice(index, 1)
        }
      }
    }
  }

  public async trigger(evt: string, ...args: any[]): Promise<void> {
    const callbacks = this.listenerMap.get(evt)
    const promises: (void | Promise<void>)[] = []
    if (callbacks) {
      for (const cb of callbacks) {
        promises.push(cb(...args))
      }
    }
    await Promise.all(promises)
  }
}
