import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const APP_READY = "app://ready";

export interface ReadyPayload {
  message: string;
}

/** Escuta o evento de prontidão emitido pelo backend (Marco 0). */
export function onReady(cb: (payload: ReadyPayload) => void): Promise<UnlistenFn> {
  return listen<ReadyPayload>(APP_READY, (event) => cb(event.payload));
}

// ── Marco 1 — Eventos de Descoberta ──────────────────────────────────────────

export interface ScanProgressPayload {
  scanId: string;
  filesFound: number;
  dirsFound: number;
  currentPath: string;
}

export interface ScanCompletedPayload {
  scanId: string;
  totalArquivos: number;
  totalDiretorios: number;
  totalErros: number;
  durationMs: number;
}

export interface IndexingProgressPayload {
  indexingId: string;
  processed: number;
  total: number;
  currentFile: string;
}

export interface IndexingCompletedPayload {
  indexingId: string;
  processados: number;
  ignorados: number;
  falhos: number;
  durationMs: number;
}

export function onScanProgress(cb: (p: ScanProgressPayload) => void): Promise<UnlistenFn> {
  return listen<ScanProgressPayload>("scan://progress", (e) => cb(e.payload));
}

export function onScanCompleted(cb: (p: ScanCompletedPayload) => void): Promise<UnlistenFn> {
  return listen<ScanCompletedPayload>("scan://completed", (e) => cb(e.payload));
}

export function onScanFailed(cb: (p: { scanId: string }) => void): Promise<UnlistenFn> {
  return listen<{ scanId: string }>("scan://failed", (e) => cb(e.payload));
}

export function onScanCancelled(cb: (p: { scanId: string }) => void): Promise<UnlistenFn> {
  return listen<{ scanId: string }>("scan://cancelled", (e) => cb(e.payload));
}

export function onIndexingProgress(cb: (p: IndexingProgressPayload) => void): Promise<UnlistenFn> {
  return listen<IndexingProgressPayload>("indexing://progress", (e) => cb(e.payload));
}

export function onIndexingCompleted(
  cb: (p: IndexingCompletedPayload) => void,
): Promise<UnlistenFn> {
  return listen<IndexingCompletedPayload>("indexing://completed", (e) => cb(e.payload));
}

export function onIndexingFailed(cb: (p: { indexingId: string }) => void): Promise<UnlistenFn> {
  return listen<{ indexingId: string }>("indexing://failed", (e) => cb(e.payload));
}

// ── Marco 2 — Eventos de Análise ─────────────────────────────────────────────

export interface AnalysisStartedPayload {
  analysisId: string;
  total: number;
}

export interface AnalysisProgressPayload {
  analysisId: string;
  processed: number;
  total: number;
  currentFile: string;
}

export interface AnalysisCompletedPayload {
  analysisId: string;
  stats: {
    processados: number;
    semConteudo: number;
    falhos: number;
    clustersCriados: number;
    durationMs: number;
  };
}

export function onAnalysisStarted(
  cb: (p: AnalysisStartedPayload) => void,
): Promise<UnlistenFn> {
  return listen<AnalysisStartedPayload>("analysis://started", (e) => cb(e.payload));
}

export function onAnalysisProgress(
  cb: (p: AnalysisProgressPayload) => void,
): Promise<UnlistenFn> {
  return listen<AnalysisProgressPayload>("analysis://progress", (e) => cb(e.payload));
}

export function onAnalysisCompleted(
  cb: (p: AnalysisCompletedPayload) => void,
): Promise<UnlistenFn> {
  return listen<AnalysisCompletedPayload>("analysis://completed", (e) => cb(e.payload));
}

export function onAnalysisFailed(
  cb: (p: { analysisId: string; error: string }) => void,
): Promise<UnlistenFn> {
  return listen<{ analysisId: string; error: string }>("analysis://failed", (e) =>
    cb(e.payload),
  );
}
