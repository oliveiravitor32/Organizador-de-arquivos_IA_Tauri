import { invoke } from "@tauri-apps/api/core";

/** Erro estruturado retornado pelo backend (ver contratos-tauri.md). */
export interface AppError {
  code: string;
  message: string;
  details: unknown | null;
}

/** Command de teste do Marco 0: retorna a versão da aplicação. */
export function ping(): Promise<string> {
  return invoke<string>("ping");
}
