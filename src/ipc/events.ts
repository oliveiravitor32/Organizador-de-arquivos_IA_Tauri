import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const APP_READY = "app://ready";

export interface ReadyPayload {
  message: string;
}

/** Escuta o evento de prontidão emitido pelo backend (Marco 0). */
export function onReady(cb: (payload: ReadyPayload) => void): Promise<UnlistenFn> {
  return listen<ReadyPayload>(APP_READY, (event) => cb(event.payload));
}
