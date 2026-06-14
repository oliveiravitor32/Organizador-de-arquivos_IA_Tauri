import { useEffect, useRef } from "react";

import type { UnlistenFn } from "@tauri-apps/api/event";

import { Button } from "@/components/ui/button";
import { useTranslations } from "@/i18n";
import { analisarArquivos } from "@/ipc/commands";
import {
  onAnalysisCompleted,
  onAnalysisFailed,
  onAnalysisProgress,
  onAnalysisStarted,
} from "@/ipc/events";
import { useAnaliseStore } from "@/stores/analise";

export function Analise() {
  const t = useTranslations();
  const a = t.analise;
  const store = useAnaliseStore();
  const unlistenRefs = useRef<UnlistenFn[]>([]);

  useEffect(() => {
    let montado = true;

    void (async () => {
      const fns = await Promise.all([
        onAnalysisStarted((p) => {
          if (montado) store.setStarted(p.analysisId, p.total);
        }),
        onAnalysisProgress((p) => {
          if (montado) store.setProgress(p.processed, p.total, p.currentFile);
        }),
        onAnalysisCompleted((p) => {
          if (montado) store.setCompleted(p.stats);
        }),
        onAnalysisFailed((p) => {
          if (montado) store.setError(p.error ?? a.erro);
        }),
      ]);
      if (montado) unlistenRefs.current = fns;
    })();

    return () => {
      montado = false;
      unlistenRefs.current.forEach((fn) => fn());
    };
  }, []);

  async function handleAnalisar() {
    store.reset();
    try {
      await analisarArquivos();
    } catch (err) {
      store.setError(err instanceof Error ? err.message : a.erro);
    }
  }

  const { status, progress, stats, erro } = store;
  const analisando = status === "analysing";
  const concluido = status === "done";

  return (
    <section aria-label={a.titulo} className="space-y-4 p-4">
      <h2 className="text-lg font-semibold text-foreground">{a.titulo}</h2>

      <Button onClick={handleAnalisar} disabled={analisando}>
        {analisando ? a.analisando : a.analisar}
      </Button>

      {analisando && progress && (
        <p className="text-sm text-muted-foreground">
          {progress.processed}/{progress.total} — {progress.currentFile}
        </p>
      )}

      {concluido && stats && (
        <div className="text-sm text-foreground space-y-1" data-testid="analise-stats">
          <p>{a.concluido}</p>
          <p>{stats.processados} {a.processados}</p>
          <p>{stats.clustersCriados} {a.clusters}</p>
          {stats.semConteudo > 0 && (
            <p className="text-muted-foreground">
              {stats.semConteudo} {a.semConteudo}
            </p>
          )}
          {stats.falhos > 0 && (
            <p className="text-destructive">
              {stats.falhos} {a.falhos}
            </p>
          )}
        </div>
      )}

      {status === "error" && erro && (
        <p role="alert" className="text-sm text-destructive">
          {erro}
        </p>
      )}
    </section>
  );
}
