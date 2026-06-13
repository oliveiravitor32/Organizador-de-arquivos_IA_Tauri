import { describe, it, expect, beforeEach } from "vitest";
import { useDescobertaStore } from "./descoberta";

beforeEach(() => {
  useDescobertaStore.getState().reset();
});

describe("descoberta store — estado inicial", () => {
  it("começa com status idle e todos os campos nulos", () => {
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("idle");
    expect(s.scanId).toBeNull();
    expect(s.indexingId).toBeNull();
    expect(s.scanStats).toBeNull();
    expect(s.indexingStats).toBeNull();
    expect(s.scanProgress).toBeNull();
    expect(s.indexingProgress).toBeNull();
    expect(s.erro).toBeNull();
  });
});

describe("descoberta store — setScanStarted", () => {
  it("muda status para scanning e grava scanId", () => {
    useDescobertaStore.getState().setScanStarted("scan-abc");
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("scanning");
    expect(s.scanId).toBe("scan-abc");
    expect(s.scanProgress).toBeNull();
    expect(s.erro).toBeNull();
  });
});

describe("descoberta store — setScanProgress", () => {
  it("atualiza filesFound e dirsFound sem mudar o status", () => {
    useDescobertaStore.getState().setScanStarted("scan-1");
    useDescobertaStore.getState().setScanProgress(15, 3);
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("scanning");
    expect(s.scanProgress).toEqual({ filesFound: 15, dirsFound: 3 });
  });
});

describe("descoberta store — setScanCompleted", () => {
  it("muda status para scan_done e armazena as stats", () => {
    const stats = {
      scanId: "scan-1",
      totalArquivos: 42,
      totalDiretorios: 7,
      totalErros: 0,
      durationMs: 1500,
    };
    useDescobertaStore.getState().setScanCompleted(stats);
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("scan_done");
    expect(s.scanStats).toEqual(stats);
  });
});

describe("descoberta store — setIndexingStarted", () => {
  it("muda status para indexing e grava indexingId", () => {
    useDescobertaStore.getState().setIndexingStarted("idx-xyz");
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("indexing");
    expect(s.indexingId).toBe("idx-xyz");
    expect(s.indexingProgress).toBeNull();
  });
});

describe("descoberta store — setIndexingProgress", () => {
  it("atualiza processed e total sem mudar o status", () => {
    useDescobertaStore.getState().setIndexingStarted("idx-1");
    useDescobertaStore.getState().setIndexingProgress(10, 50);
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("indexing");
    expect(s.indexingProgress).toEqual({ processed: 10, total: 50 });
  });
});

describe("descoberta store — setIndexingCompleted", () => {
  it("muda status para indexing_done e armazena as stats", () => {
    const stats = {
      indexingId: "idx-1",
      processados: 40,
      ignorados: 1,
      falhos: 1,
      durationMs: 3000,
    };
    useDescobertaStore.getState().setIndexingCompleted(stats);
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("indexing_done");
    expect(s.indexingStats).toEqual(stats);
  });
});

describe("descoberta store — setError", () => {
  it("muda status para error e armazena a mensagem", () => {
    useDescobertaStore.getState().setError("Erro ao escanear");
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("error");
    expect(s.erro).toBe("Erro ao escanear");
  });
});

describe("descoberta store — setCancelled", () => {
  it("muda status para cancelled", () => {
    useDescobertaStore.getState().setScanStarted("scan-1");
    useDescobertaStore.getState().setCancelled();
    expect(useDescobertaStore.getState().status).toBe("cancelled");
  });
});

describe("descoberta store — reset", () => {
  it("restaura todos os campos ao estado inicial", () => {
    const store = useDescobertaStore.getState();
    store.setScanStarted("scan-x");
    store.setScanProgress(5, 2);
    store.setError("ops");
    store.reset();
    const s = useDescobertaStore.getState();
    expect(s.status).toBe("idle");
    expect(s.scanId).toBeNull();
    expect(s.scanProgress).toBeNull();
    expect(s.erro).toBeNull();
  });
});
