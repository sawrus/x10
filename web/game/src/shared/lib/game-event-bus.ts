import type { GameEventMap } from '../model'

type GameEventKey = keyof GameEventMap
type GameEventHandler<TKey extends GameEventKey> = (payload: GameEventMap[TKey]) => void

export type TypedGameEventBus = {
  emit<TKey extends GameEventKey>(eventName: TKey, payload: GameEventMap[TKey]): void
  off<TKey extends GameEventKey>(eventName: TKey, handler: GameEventHandler<TKey>): void
  on<TKey extends GameEventKey>(eventName: TKey, handler: GameEventHandler<TKey>): () => void
}

// Example:
// const stop = gameEventBus.on('sphere:highlight', ({ sphereId }) => void sphereId)
// gameEventBus.emit('sphere:highlight', { active: true, sphereId: 'focus', source: 'system' })
// stop()
export function createGameEventBus(): TypedGameEventBus {
  const listeners = new Map<GameEventKey, Set<GameEventHandler<GameEventKey>>>()

  return {
    emit(eventName, payload) {
      const handlers = listeners.get(eventName)
      if (!handlers) {
        return
      }

      handlers.forEach((handler) => {
        handler(payload)
      })
    },
    off(eventName, handler) {
      const handlers = listeners.get(eventName)

      if (!handlers) {
        return
      }

      handlers.delete(handler as GameEventHandler<GameEventKey>)

      if (handlers.size === 0) {
        listeners.delete(eventName)
      }
    },
    on(eventName, handler) {
      const handlers = listeners.get(eventName) ?? new Set<GameEventHandler<GameEventKey>>()

      handlers.add(handler as GameEventHandler<GameEventKey>)
      listeners.set(eventName, handlers)

      return () => {
        handlers.delete(handler as GameEventHandler<GameEventKey>)

        if (handlers.size === 0) {
          listeners.delete(eventName)
        }
      }
    },
  }
}

export const gameEventBus = createGameEventBus()
