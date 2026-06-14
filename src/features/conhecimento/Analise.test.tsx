import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor, act } from "@testing-library/react";

vi.mock("@/ipc/commands", () => ({
  analisarArquivos: vi.fn(),
}));

vi.mock("@/ipc/events", () => ({
  onAnalysisStarted: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisProgress: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisCompleted: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisFailed: vi.fn(() => Promise.resolve(() => {})),
}));

import { analisarArquivos } from "@/ipc/commands";
import {
  onAnalysisCompleted,
  onAnalysisFailed,
  onAnalysisStarted,
} from "@/ipc/events";
import { useAnaliseStore } from "@/stores/analise";
import { Analise } from "./Analise";

function renderAnalise() {
  return render(<Analise />);
}

describe("Analise — estado inicial", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useAnaliseStore.getState().reset();
  });

  it("exibe botão de analisar", () => {
    renderAnalise();
    expect(screen.getByRole("button", { name: /analisar/i })).toBeInTheDocument();
  });

  it("não exibe stats antes de analisar", () => {
    renderAnalise();
    expect(screen.queryByTestId("analise-stats")).not.toBeInTheDocument();
  });
});

describe("Analise — UC-003 CA-001: iniciar análise", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useAnaliseStore.getState().reset();
  });

  it("chama analisarArquivos ao clicar no botão", async () => {
    vi.mocked(analisarArquivos).mockResolvedValue({ analysisId: "a1" });
    renderAnalise();
    fireEvent.click(screen.getByRole("button", { name: /analisar/i }));
    await waitFor(() => expect(analisarArquivos).toHaveBeenCalledTimes(1));
  });

  it("botão fica desabilitado durante a análise", async () => {
    vi.mocked(analisarArquivos).mockResolvedValue({ analysisId: "a1" });
    renderAnalise();
    await act(async () => {
      useAnaliseStore.getState().setStarted("a1", 5);
    });
    expect(screen.getByRole("button")).toBeDisabled();
  });
});

describe("Analise — UC-003 CA-002: análise concluída", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useAnaliseStore.getState().reset();
  });

  it("exibe stats após análise concluída", async () => {
    renderAnalise();
    // Captura callback registrado pelo componente
    const completedCb = vi.mocked(onAnalysisCompleted).mock.calls[0]?.[0];
    await act(async () => {
      completedCb?.({
        analysisId: "a1",
        stats: {
          processados: 10,
          semConteudo: 2,
          falhos: 0,
          clustersCriados: 3,
          durationMs: 800,
        },
      });
    });
    expect(screen.getByTestId("analise-stats")).toBeInTheDocument();
    expect(screen.getByText(/10 arquivos processados/i)).toBeInTheDocument();
    expect(screen.getByText(/3 clusters formados/i)).toBeInTheDocument();
  });
});

describe("Analise — UC-003 CA-003: falha na análise", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useAnaliseStore.getState().reset();
  });

  it("exibe erro quando onAnalysisFailed dispara", async () => {
    renderAnalise();
    const failedCb = vi.mocked(onAnalysisFailed).mock.calls[0]?.[0];
    await act(async () => {
      failedCb?.({ analysisId: "a1", error: "Ollama indisponível" });
    });
    expect(screen.getByRole("alert")).toHaveTextContent(/Ollama indisponível/i);
  });

  it("exibe erro quando analisarArquivos rejeita", async () => {
    vi.mocked(analisarArquivos).mockRejectedValue(new Error("conexão recusada"));
    renderAnalise();
    fireEvent.click(screen.getByRole("button", { name: /analisar/i }));
    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent(/conexão recusada/i),
    );
  });
});

describe("Analise — UC-003 CA-004: sem arquivos pendentes", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useAnaliseStore.getState().reset();
  });

  it("exibe zero processados quando backend retorna stats zeradas", async () => {
    renderAnalise();
    const startedCb = vi.mocked(onAnalysisStarted).mock.calls[0]?.[0];
    const completedCb = vi.mocked(onAnalysisCompleted).mock.calls[0]?.[0];
    await act(async () => {
      startedCb?.({ analysisId: "a2", total: 0 });
      completedCb?.({
        analysisId: "a2",
        stats: { processados: 0, semConteudo: 0, falhos: 0, clustersCriados: 0, durationMs: 5 },
      });
    });
    expect(screen.getByText(/0 arquivos processados/i)).toBeInTheDocument();
  });
});
