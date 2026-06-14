import { beforeEach, describe, expect, it } from "vitest";

import { useSugestoesStore } from "./sugestoes";

describe("useSugestoesStore", () => {
  beforeEach(() => {
    useSugestoesStore.getState().reset();
  });

  it("estado inicial é idle sem dados", () => {
    const s = useSugestoesStore.getState();
    expect(s.status).toBe("idle");
    expect(s.generationId).toBeNull();
    expect(s.sugestoes).toHaveLength(0);
    expect(s.stats).toBeNull();
    expect(s.erro).toBeNull();
  });

  it("setStarted transita para generating e limpa sugestoes anteriores", () => {
    useSugestoesStore.getState().addSugestao({ id: "x", tipo: "agrupamento", titulo: "X", confianca: 0.9, status: "pendente" });
    useSugestoesStore.getState().setStarted("g1", 3);
    const s = useSugestoesStore.getState();
    expect(s.status).toBe("generating");
    expect(s.generationId).toBe("g1");
    expect(s.sugestoes).toHaveLength(0);
    expect(s.erro).toBeNull();
  });

  it("addSugestao acumula sugestões em ordem", () => {
    useSugestoesStore.getState().setStarted("g2", 2);
    useSugestoesStore.getState().addSugestao({ id: "a", tipo: "agrupamento", titulo: "A", confianca: 0.9, status: "pendente" });
    useSugestoesStore.getState().addSugestao({ id: "b", tipo: "agrupamento", titulo: "B", confianca: 0.7, status: "pendente" });
    expect(useSugestoesStore.getState().sugestoes).toHaveLength(2);
  });

  it("setCompleted transita para done com stats", () => {
    useSugestoesStore.getState().setStarted("g3", 1);
    useSugestoesStore.getState().setCompleted({ geradas: 1, descartadas: 0, durationMs: 500 });
    const s = useSugestoesStore.getState();
    expect(s.status).toBe("done");
    expect(s.stats?.geradas).toBe(1);
    expect(s.stats?.durationMs).toBe(500);
  });

  it("setError transita para error com mensagem", () => {
    useSugestoesStore.getState().setError("Ollama indisponível");
    const s = useSugestoesStore.getState();
    expect(s.status).toBe("error");
    expect(s.erro).toBe("Ollama indisponível");
  });

  it("reset volta para estado inicial", () => {
    useSugestoesStore.getState().setStarted("g4", 5);
    useSugestoesStore.getState().reset();
    const s = useSugestoesStore.getState();
    expect(s.status).toBe("idle");
    expect(s.generationId).toBeNull();
    expect(s.sugestoes).toHaveLength(0);
  });
});
