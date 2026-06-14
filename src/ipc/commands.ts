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

/** Pede ao backend que emita o evento de prontidão (após o listener existir). */
export function announceReady(): Promise<void> {
  return invoke<void>("announce_ready");
}

// ── Marco 1 — Descoberta ──────────────────────────────────────────────────────

export interface EscanearDiretorioResult {
  scanId: string;
}

export interface IndexarArquivosResult {
  indexingId: string;
}

export interface CancelarOperacaoResult {
  operationId: string;
  status: "cancelando" | "nao_encontrada";
}

/** Inicia o escaneamento recursivo de um diretório (UC-001). */
export function escanearDiretorio(
  rootPath: string,
  ignore: string[] = [],
): Promise<EscanearDiretorioResult> {
  return invoke<EscanearDiretorioResult>("escanear_diretorio", {
    rootPath,
    ignore,
  });
}

/** Inicia a indexação dos arquivos descobertos (UC-002). */
export function indexarArquivos(
  scanId: string,
): Promise<IndexarArquivosResult> {
  return invoke<IndexarArquivosResult>("indexar_arquivos", { scanId });
}

/** Retorna o resultado de uma indexação concluída, se disponível (CA-HMR-001). */
export function consultarIndexacao(
  indexingId: string,
): Promise<IndexingCompletedResult | null> {
  return invoke<IndexingCompletedResult | null>("consultar_indexacao", {
    indexingId,
  });
}

export interface IndexingCompletedResult {
  indexingId: string;
  processados: number;
  ignorados: number;
  falhos: number;
  durationMs: number;
}

/** Cancela uma operação assíncrona em andamento. */
export function cancelarOperacao(
  operationId: string,
): Promise<CancelarOperacaoResult> {
  return invoke<CancelarOperacaoResult>("cancelar_operacao", { operationId });
}

// ── Marco 2 — Conhecimento ────────────────────────────────────────────────────

export interface AnalisarArquivosResult {
  analysisId: string;
}

/** Inicia o pipeline de análise semântica dos arquivos pendentes (UC-003). */
export function analisarArquivos(
  fileIds?: string[],
): Promise<AnalisarArquivosResult> {
  return invoke<AnalisarArquivosResult>("analisar_arquivos", {
    fileIds: fileIds ?? null,
  });
}

// ── Marco 3 — Sugestões ───────────────────────────────────────────────────────

export interface GerarSugestoesResult {
  suggestionGenerationId: string;
}

export interface ExplicarSugestaoResult {
  suggestionId: string;
  tipo: string;
  titulo: string | null;
  justificativa: string;
  evidencias: Array<{ tipo: string; valor: string }>;
  confianca: number;
  desatualizada: boolean;
  operacoes: number;
  arquivos: Array<{ id: string; nome: string; caminho: string }>;
}

export interface SugestaoItem {
  id: string;
  tipo: string;
  titulo: string | null;
  confianca: number | null;
  status: string;
}

export interface ListarSugestoesResult {
  sugestoes: SugestaoItem[];
}

/** Inicia a geração de sugestões a partir dos clusters existentes (UC-005). */
export function gerarSugestoes(): Promise<GerarSugestoesResult> {
  return invoke<GerarSugestoesResult>("gerar_sugestoes");
}

/** Retorna a explicação detalhada de uma sugestão (UC-012). */
export function explicarSugestao(suggestionId: string): Promise<ExplicarSugestaoResult> {
  return invoke<ExplicarSugestaoResult>("explicar_sugestao", { suggestionId });
}

/** Lista sugestões, opcionalmente filtradas por status. */
export function listarSugestoes(status?: string): Promise<ListarSugestoesResult> {
  return invoke<ListarSugestoesResult>("listar_sugestoes", { status: status ?? null });
}
