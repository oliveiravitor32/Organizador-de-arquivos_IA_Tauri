import { useEffect, useRef, useState } from "react";

import { open } from "@tauri-apps/plugin-dialog";
import type { UnlistenFn } from "@tauri-apps/api/event";

import { Button } from "@/components/ui/button";
import { useTranslations } from "@/i18n";
import {
  cancelarOperacao,
  consultarIndexacao,
  escanearDiretorio,
  indexarArquivos,
} from "@/ipc/commands";
import {
  onIndexingCompleted,
  onIndexingFailed,
  onIndexingProgress,
  onScanCancelled,
  onScanCompleted,
  onScanFailed,
  onScanProgress,
} from "@/ipc/events";
import { useDescobertaStore } from "@/stores/descoberta";

export function Descoberta() {
  const t = useTranslations();
  const d = t.descoberta;
  const store = useDescobertaStore();
  const [diretorio, setDiretorio] = useState<string | null>(null);
  const unlistenRefs = useRef<UnlistenFn[]>([]);

  // Registra todos os listeners de eventos e limpa no unmount.
  useEffect(() => {
    let montado = true;

    void (async () => {
      const fns = await Promise.all([
        onScanProgress((p) => {
          if (montado) store.setScanProgress(p.filesFound, p.dirsFound);
        }),
        onScanCompleted((p) => {
          if (montado) store.setScanCompleted(p);
        }),
        onScanFailed(() => {
          if (montado) store.setError(d.erroScan);
        }),
        onScanCancelled(() => {
          if (montado) store.setCancelled();
        }),
        onIndexingProgress((p) => {
          if (montado) store.setIndexingProgress(p.processed, p.total);
        }),
        onIndexingCompleted((p) => {
          if (montado) store.setIndexingCompleted(p);
        }),
        onIndexingFailed(() => {
          if (montado) store.setError(d.erroIndexacao);
        }),
      ]);
      unlistenRefs.current = fns;

      // CA-HMR-001: reconexão após hot-reload.
      // Listeners já registrados — consulta o backend pelo resultado perdido.
      const activeId = localStorage.getItem("activeIndexingId");
      if (activeId && montado) {
        const resultado = await consultarIndexacao(activeId);
        if (resultado && montado) {
          store.setIndexingCompleted(resultado);
        }
      }
    })();

    return () => {
      montado = false;
      unlistenRefs.current.forEach((fn) => fn());
    };
  }, [store, d]);

  async function selecionarDiretorio() {
    const selecionado = await open({ directory: true, multiple: false });
    if (typeof selecionado === "string") {
      setDiretorio(selecionado);
      store.reset();
    }
  }

  async function iniciarScan() {
    if (!diretorio) return;
    try {
      const { scanId } = await escanearDiretorio(diretorio);
      store.setScanStarted(scanId);
    } catch {
      store.setError(d.erroScan);
    }
  }

  async function iniciarIndexacao() {
    if (!store.scanId) return;
    try {
      const { indexingId } = await indexarArquivos(store.scanId);
      store.setIndexingStarted(indexingId);
    } catch {
      store.setError(d.erroIndexacao);
    }
  }

  async function cancelar() {
    const operationId = store.indexingId ?? store.scanId;
    if (operationId) {
      await cancelarOperacao(operationId);
    }
  }

  const estaAtivo = store.status === "scanning" || store.status === "indexing";

  return (
    <section className="flex w-full max-w-lg flex-col gap-6">
      <h2 className="text-xl font-semibold text-foreground">{d.titulo}</h2>

      {/* Seleção de diretório */}
      <div className="flex flex-col gap-2">
        <Button variant="outline" onClick={selecionarDiretorio} disabled={estaAtivo}>
          {d.selecionarDiretorio}
        </Button>
        {diretorio ? (
          <p className="truncate text-sm text-muted-foreground">
            {d.dirSelecionado}: <span className="text-foreground">{diretorio}</span>
          </p>
        ) : (
          <p className="text-sm text-muted-foreground">{d.nenhumDir}</p>
        )}
      </div>

      {/* Botão de scan */}
      {diretorio && store.status === "idle" && (
        <Button onClick={iniciarScan}>{d.escanear}</Button>
      )}

      {/* Progresso do scan */}
      {store.status === "scanning" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">{d.escaneando}</p>
          {store.scanProgress && (
            <p className="text-sm text-foreground">
              {store.scanProgress.filesFound} {d.arquivosEncontrados},{" "}
              {store.scanProgress.dirsFound} {d.diretoriosEncontrados}
            </p>
          )}
          <Button variant="outline" onClick={cancelar}>
            {d.cancelar}
          </Button>
        </div>
      )}

      {/* Resultado do scan + botão de indexação */}
      {store.status === "scan_done" && store.scanStats && (
        <div className="flex flex-col gap-3">
          <p className="text-sm font-medium text-foreground">{d.resultadoScan}</p>
          <p className="text-sm text-muted-foreground">
            {store.scanStats.totalArquivos} {d.arquivosEncontrados},{" "}
            {store.scanStats.totalDiretorios} {d.diretoriosEncontrados}
          </p>
          <Button onClick={iniciarIndexacao}>{d.indexar}</Button>
        </div>
      )}

      {/* Progresso da indexação */}
      {store.status === "indexing" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">{d.indexando}</p>
          {store.indexingProgress && (
            <p className="text-sm text-foreground">
              {store.indexingProgress.processed}/{store.indexingProgress.total}{" "}
              {d.processados}
            </p>
          )}
          <Button variant="outline" onClick={cancelar}>
            {d.cancelar}
          </Button>
        </div>
      )}

      {/* Resultado da indexação */}
      {store.status === "indexing_done" && store.indexingStats && (
        <div className="flex flex-col gap-3">
          <p className="text-sm font-medium text-foreground">{d.resultadoIndexacao}</p>
          <p className="text-sm text-muted-foreground">
            {store.indexingStats.processados} {d.processados},{" "}
            {store.indexingStats.falhos} {d.falhos}
          </p>
          <Button variant="outline" onClick={() => { store.reset(); setDiretorio(null); }}>
            {d.novoEscaneamento}
          </Button>
        </div>
      )}

      {/* Cancelado */}
      {store.status === "cancelled" && (
        <div className="flex flex-col gap-3">
          <p className="text-sm text-muted-foreground">{d.cancelado}</p>
          <Button variant="outline" onClick={() => { store.reset(); setDiretorio(null); }}>
            {d.novoEscaneamento}
          </Button>
        </div>
      )}

      {/* Erro */}
      {store.status === "error" && (
        <div className="flex flex-col gap-3">
          <p className="text-sm text-destructive">{store.erro}</p>
          <Button variant="outline" onClick={() => { store.reset(); }}>
            {d.novoEscaneamento}
          </Button>
        </div>
      )}
    </section>
  );
}
