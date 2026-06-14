import { beforeEach, describe, expect, it } from "vitest";

import { useAnaliseStore } from "./analise";

describe("useAnaliseStore", () => {
  beforeEach(() => {
    useAnaliseStore.getState().reset();
  });

  it("estado inicial é idle sem dados", () => {
    const s = useAnaliseStore.getState();
    expect(s.status).toBe("idle");
    expect(s.analysisId).toBeNull();
    expect(s.progress).toBeNull();
    expect(s.stats).toBeNull();
    expect(s.erro).toBeNull();
  });

  it("setStarted transita para analysing com total", () => {
    useAnaliseStore.getState().setStarted("a1", 10);
    const s = useAnaliseStore.getState();
    expect(s.status).toBe("analysing");
    expect(s.analysisId).toBe("a1");
    expect(s.progress?.total).toBe(10);
    expect(s.progress?.processed).toBe(0);
    expect(s.erro).toBeNull();
  });

  it("setProgress atualiza progresso", () => {
    useAnaliseStore.getState().setStarted("a2", 5);
    useAnaliseStore.getState().setProgress(3, 5, "arquivo.pdf");
    const s = useAnaliseStore.getState();
    expect(s.progress?.processed).toBe(3);
    expect(s.progress?.currentFile).toBe("arquivo.pdf");
  });

  it("setCompleted transita para done com stats", () => {
    const stats = {
      processados: 10,
      semConteudo: 2,
      falhos: 1,
      clustersCriados: 3,
      durationMs: 1200,
    };
    useAnaliseStore.getState().setCompleted(stats);
    const s = useAnaliseStore.getState();
    expect(s.status).toBe("done");
    expect(s.stats?.processados).toBe(10);
    expect(s.stats?.semConteudo).toBe(2);
    expect(s.stats?.clustersCriados).toBe(3);
  });

  it("setError transita para error com mensagem", () => {
    useAnaliseStore.getState().setError("Ollama indisponível");
    const s = useAnaliseStore.getState();
    expect(s.status).toBe("error");
    expect(s.erro).toBe("Ollama indisponível");
  });

  it("reset volta para estado inicial", () => {
    useAnaliseStore.getState().setStarted("a3", 5);
    useAnaliseStore.getState().reset();
    const s = useAnaliseStore.getState();
    expect(s.status).toBe("idle");
    expect(s.analysisId).toBeNull();
  });
});
