import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { useTranslations } from "@/i18n";
import { type ExplicarSugestaoResult, explicarSugestao } from "@/ipc/commands";

interface Props {
  suggestionId: string;
  onFechar: () => void;
}

export function DetalhesSugestao({ suggestionId, onFechar }: Props) {
  const t = useTranslations();
  const d = t.sugestoes.detalhe;

  const [dados, setDados] = useState<ExplicarSugestaoResult | null>(null);
  const [erro, setErro] = useState<string | null>(null);

  useEffect(() => {
    setDados(null);
    setErro(null);
    explicarSugestao(suggestionId)
      .then(setDados)
      .catch((e: unknown) =>
        setErro(e instanceof Error ? e.message : "Erro ao carregar detalhes"),
      );
  }, [suggestionId]);

  return (
    <div
      role="dialog"
      aria-label={d.titulo}
      className="rounded border border-border p-4 space-y-3 bg-background"
      data-testid="detalhe-sugestao"
    >
      <h3 className="font-semibold text-foreground">{d.titulo}</h3>

      {erro && (
        <p role="alert" className="text-sm text-destructive">
          {erro}
        </p>
      )}

      {dados && (
        <>
          {dados.desatualizada && (
            <p className="text-xs text-destructive" data-testid="badge-desatualizada">
              {d.desatualizada}
            </p>
          )}

          <div>
            <p className="text-xs font-medium text-muted-foreground uppercase">{d.confianca}</p>
            <p className="text-sm text-foreground" data-testid="confianca-valor">
              {(dados.confianca * 100).toFixed(0)}%
            </p>
          </div>

          <div>
            <p className="text-xs font-medium text-muted-foreground uppercase">{d.justificativa}</p>
            <p className="text-sm text-foreground" data-testid="justificativa">
              {dados.justificativa}
            </p>
          </div>

          {dados.evidencias.length > 0 && (
            <div>
              <p className="text-xs font-medium text-muted-foreground uppercase">{d.evidencias}</p>
              <ul className="space-y-1" data-testid="lista-evidencias">
                {dados.evidencias.map((ev, i) => (
                  <li key={i} className="text-sm text-foreground">
                    {ev.tipo}: {ev.valor}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {dados.arquivos.length > 0 && (
            <div>
              <p className="text-xs font-medium text-muted-foreground uppercase">{d.arquivos}</p>
              <ul className="space-y-1" data-testid="lista-arquivos">
                {dados.arquivos.map((arq) => (
                  <li
                    key={arq.id}
                    className="text-sm text-foreground"
                    title={arq.caminho}
                  >
                    {arq.nome}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </>
      )}

      <Button variant="outline" onClick={onFechar}>
        {d.fechar}
      </Button>
    </div>
  );
}
