import { useEffect, useRef, useState } from "react";

import type { UnlistenFn } from "@tauri-apps/api/event";

import { Button } from "@/components/ui/button";
import { useTranslations } from "@/i18n";
import type { SugestaoItem } from "@/ipc/commands";
import { gerarSugestoes } from "@/ipc/commands";
import {
  onSuggestionCompleted,
  onSuggestionCreated,
  onSuggestionFailed,
  onSuggestionStarted,
} from "@/ipc/events";
import { useSugestoesStore } from "@/stores/sugestoes";
import { DetalhesSugestao } from "./DetalhesSugestao";

function badgeConfianca(confianca: number | null, labels: { alta: string; media: string; baixa: string }) {
  if (confianca === null) return null;
  if (confianca >= 0.75) return { label: labels.alta, cls: "text-foreground" };
  if (confianca >= 0.50) return { label: labels.media, cls: "text-muted-foreground" };
  return { label: labels.baixa, cls: "text-destructive" };
}

export function Sugestoes() {
  const t = useTranslations();
  const s = t.sugestoes;
  const store = useSugestoesStore();
  const unlistenRefs = useRef<UnlistenFn[]>([]);
  const [selecionada, setSelecionada] = useState<SugestaoItem | null>(null);

  useEffect(() => {
    let montado = true;

    void (async () => {
      const fns = await Promise.all([
        onSuggestionStarted((p) => {
          if (montado) store.setStarted(p.suggestionGenerationId, p.total);
        }),
        onSuggestionCreated((p) => {
          if (montado)
            store.addSugestao({
              id: p.suggestionId,
              tipo: "agrupamento",
              titulo: p.titulo,
              confianca: p.confianca,
              status: "pendente",
            });
        }),
        onSuggestionCompleted((p) => {
          if (montado) store.setCompleted(p.stats);
        }),
        onSuggestionFailed((p) => {
          if (montado) store.setError(p.error ?? s.erro);
        }),
      ]);
      if (montado) unlistenRefs.current = fns;
    })();

    return () => {
      montado = false;
      unlistenRefs.current.forEach((fn) => fn());
    };
  }, []);

  async function handleGerar() {
    store.reset();
    setSelecionada(null);
    try {
      await gerarSugestoes();
    } catch (err) {
      store.setError(err instanceof Error ? err.message : s.erro);
    }
  }

  const { status, sugestoes, stats, erro } = store;
  const gerando = status === "generating";
  const concluido = status === "done";

  return (
    <section aria-label={s.titulo} className="space-y-4 p-4">
      <h2 className="text-lg font-semibold text-foreground">{s.titulo}</h2>

      <Button onClick={handleGerar} disabled={gerando}>
        {gerando ? s.gerando : s.gerar}
      </Button>

      {gerando && sugestoes.length > 0 && (
        <p className="text-sm text-muted-foreground">
          {sugestoes.length} {s.geradas}…
        </p>
      )}

      {concluido && stats && (
        <p className="text-sm text-muted-foreground">
          {s.concluido} — {stats.geradas} {s.geradas}, {stats.descartadas} {s.descartadas}
        </p>
      )}

      {status === "error" && erro && (
        <p role="alert" className="text-sm text-destructive">
          {erro}
        </p>
      )}

      {sugestoes.length > 0 && (
        <ul className="space-y-2" data-testid="lista-sugestoes">
          {sugestoes.map((sg) => {
            const badge = badgeConfianca(sg.confianca, {
              alta: s.confiancaAlta,
              media: s.confiancaMedia,
              baixa: s.confiancaBaixa,
            });
            return (
              <li key={sg.id}>
                <button
                  type="button"
                  onClick={() => setSelecionada(sg)}
                  className="w-full text-left rounded border border-border p-3 hover:bg-accent transition-colors"
                  data-testid={`sugestao-${sg.id}`}
                >
                  <span className="block text-sm font-medium text-foreground">
                    {sg.titulo ?? sg.id}
                  </span>
                  {badge && (
                    <span className={`text-xs ${badge.cls}`}>{badge.label}</span>
                  )}
                </button>
              </li>
            );
          })}
        </ul>
      )}

      {sugestoes.length === 0 && (status === "done" || status === "idle") && (
        <p className="text-sm text-muted-foreground" data-testid="vazio">
          {s.vazio}
        </p>
      )}

      {selecionada && (
        <DetalhesSugestao
          suggestionId={selecionada.id}
          onFechar={() => setSelecionada(null)}
        />
      )}
    </section>
  );
}
