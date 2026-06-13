import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor, act } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

// Mocks de IPC — o componente nunca chama o backend real em testes unitários.
vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

vi.mock("@/ipc/commands", () => ({
  ping: vi.fn(() => Promise.resolve("0.1.0")),
  announceReady: vi.fn(() => Promise.resolve()),
  escanearDiretorio: vi.fn(),
  indexarArquivos: vi.fn(),
  cancelarOperacao: vi.fn(() => Promise.resolve({ operationId: "op-1", status: "cancelando" })),
}));

vi.mock("@/ipc/events", () => ({
  onReady: vi.fn(() => Promise.resolve(() => {})),
  onScanProgress: vi.fn(() => Promise.resolve(() => {})),
  onScanCompleted: vi.fn(() => Promise.resolve(() => {})),
  onScanFailed: vi.fn(() => Promise.resolve(() => {})),
  onScanCancelled: vi.fn(() => Promise.resolve(() => {})),
  onIndexingProgress: vi.fn(() => Promise.resolve(() => {})),
  onIndexingCompleted: vi.fn(() => Promise.resolve(() => {})),
  onIndexingFailed: vi.fn(() => Promise.resolve(() => {})),
}));

import { open } from "@tauri-apps/plugin-dialog";
import { escanearDiretorio, indexarArquivos } from "@/ipc/commands";
import {
  onScanCompleted,
  onScanFailed,
  onScanCancelled,
  onIndexingCompleted,
  onIndexingFailed,
} from "@/ipc/events";
import { useDescobertaStore } from "@/stores/descoberta";
import { Descoberta } from "./Descoberta";

function renderDescoberta() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <Descoberta />
    </QueryClientProvider>,
  );
}

describe("Descoberta — UC-001 CA-001: seleção de diretório", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe mensagem de nenhum diretório por padrão", () => {
    renderDescoberta();
    expect(screen.getByText(/nenhum diretório selecionado/i)).toBeInTheDocument();
  });

  it("exibe o diretório selecionado após o usuário escolher", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));

    await waitFor(() =>
      expect(screen.getByText("/home/usuario/documentos")).toBeInTheDocument(),
    );
  });
});

describe("Descoberta — UC-001 CA-003: progresso exibido durante scan", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe estado de escaneamento após iniciar", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-123" });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));

    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(screen.getByText(/escaneando/i)).toBeInTheDocument());
  });
});

describe("Descoberta — UC-001 CA-006: resultado com estatísticas", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe estatísticas após receber ScanCompleted", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-123" });

    let scanCompletedCb: ((p: unknown) => void) | undefined;
    vi.mocked(onScanCompleted).mockImplementation((cb) => {
      scanCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(scanCompletedCb).toBeDefined());

    act(() => {
      scanCompletedCb!({
        scanId: "scan-123",
        totalArquivos: 42,
        totalDiretorios: 5,
        totalErros: 0,
        durationMs: 1200,
      });
    });

    await waitFor(() =>
      expect(screen.getByText(/escaneamento concluído/i)).toBeInTheDocument(),
    );
    expect(screen.getByText(/42/)).toBeInTheDocument();
  });
});

describe("Descoberta — UC-002: indexação disponível após scan", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe botão indexar após scan concluído", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-123" });

    let scanCompletedCb: ((p: unknown) => void) | undefined;
    vi.mocked(onScanCompleted).mockImplementation((cb) => {
      scanCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(scanCompletedCb).toBeDefined());
    act(() => {
      scanCompletedCb!({
        scanId: "scan-123",
        totalArquivos: 10,
        totalDiretorios: 2,
        totalErros: 0,
        durationMs: 500,
      });
    });

    await waitFor(() =>
      expect(screen.getByRole("button", { name: /indexar arquivos/i })).toBeInTheDocument(),
    );
  });

  it("chama indexarArquivos com o scanId correto", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-abc" });
    vi.mocked(indexarArquivos).mockResolvedValue({ indexingId: "idx-1" });

    let scanCompletedCb: ((p: unknown) => void) | undefined;
    vi.mocked(onScanCompleted).mockImplementation((cb) => {
      scanCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(scanCompletedCb).toBeDefined());
    act(() => {
      scanCompletedCb!({
        scanId: "scan-abc",
        totalArquivos: 5,
        totalDiretorios: 1,
        totalErros: 0,
        durationMs: 300,
      });
    });

    await waitFor(() => screen.getByRole("button", { name: /indexar arquivos/i }));
    fireEvent.click(screen.getByRole("button", { name: /indexar arquivos/i }));

    await waitFor(() =>
      expect(indexarArquivos).toHaveBeenCalledWith("scan-abc"),
    );
  });
});

// ─── Erro de scan ─────────────────────────────────────────────────────────────

describe("Descoberta — UC-001 CA-002: erro durante scan", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe mensagem de erro quando o scan falha via evento", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-err" });

    let scanFailedCb: (() => void) | undefined;
    vi.mocked(onScanFailed).mockImplementation((cb) => {
      scanFailedCb = cb as () => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(scanFailedCb).toBeDefined());

    act(() => { scanFailedCb!(); });

    await waitFor(() =>
      expect(screen.getByText(/erro/i)).toBeInTheDocument(),
    );
  });

  it("exibe erro quando escanearDiretorio rejeita", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockRejectedValue(new Error("falha no backend"));

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() =>
      expect(screen.getByText(/erro/i)).toBeInTheDocument(),
    );
  });
});

// ─── Cancelamento ─────────────────────────────────────────────────────────────

describe("Descoberta — UC-001 CA-003: cancelamento de scan", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe mensagem de cancelado quando o evento scan://cancelled chega", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-cancel" });

    let scanCancelledCb: (() => void) | undefined;
    vi.mocked(onScanCancelled).mockImplementation((cb) => {
      scanCancelledCb = cb as () => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));

    await waitFor(() => expect(scanCancelledCb).toBeDefined());

    act(() => { scanCancelledCb!(); });

    await waitFor(() =>
      expect(screen.getByText(/cancelad/i)).toBeInTheDocument(),
    );
  });
});

// ─── Resultado da indexação ───────────────────────────────────────────────────

describe("Descoberta — UC-002 CA-006: resultado da indexação", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useDescobertaStore.getState().reset();
  });

  it("exibe estatísticas finais quando indexação é concluída", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-idx" });
    vi.mocked(indexarArquivos).mockResolvedValue({ indexingId: "idx-1" });

    let scanCompletedCb: ((p: unknown) => void) | undefined;
    let indexingCompletedCb: ((p: unknown) => void) | undefined;

    vi.mocked(onScanCompleted).mockImplementation((cb) => {
      scanCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });
    vi.mocked(onIndexingCompleted).mockImplementation((cb) => {
      indexingCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    // Chegar ao scan_done.
    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));
    await waitFor(() => expect(scanCompletedCb).toBeDefined());
    act(() => {
      scanCompletedCb!({
        scanId: "scan-idx",
        totalArquivos: 8,
        totalDiretorios: 2,
        totalErros: 0,
        durationMs: 400,
      });
    });

    // Iniciar indexação.
    await waitFor(() => screen.getByRole("button", { name: /indexar arquivos/i }));
    fireEvent.click(screen.getByRole("button", { name: /indexar arquivos/i }));
    await waitFor(() => expect(indexingCompletedCb).toBeDefined());

    act(() => {
      indexingCompletedCb!({
        indexingId: "idx-1",
        processados: 7,
        ignorados: 1,
        falhos: 0,
        durationMs: 1200,
      });
    });

    await waitFor(() =>
      expect(screen.getByText(/indexação concluída/i)).toBeInTheDocument(),
    );
    expect(screen.getByText(/7/)).toBeInTheDocument();
  });

  it("exibe erro quando indexação falha via evento", async () => {
    vi.mocked(open).mockResolvedValue("/home/usuario/documentos");
    vi.mocked(escanearDiretorio).mockResolvedValue({ scanId: "scan-idx-err" });
    vi.mocked(indexarArquivos).mockResolvedValue({ indexingId: "idx-err" });

    let scanCompletedCb: ((p: unknown) => void) | undefined;
    let indexingFailedCb: (() => void) | undefined;

    vi.mocked(onScanCompleted).mockImplementation((cb) => {
      scanCompletedCb = cb as (p: unknown) => void;
      return Promise.resolve(() => {});
    });
    vi.mocked(onIndexingFailed).mockImplementation((cb) => {
      indexingFailedCb = cb as () => void;
      return Promise.resolve(() => {});
    });

    renderDescoberta();

    fireEvent.click(screen.getByRole("button", { name: /selecionar diretório/i }));
    await waitFor(() => screen.getByRole("button", { name: /escanear/i }));
    fireEvent.click(screen.getByRole("button", { name: /escanear/i }));
    await waitFor(() => expect(scanCompletedCb).toBeDefined());
    act(() => {
      scanCompletedCb!({
        scanId: "scan-idx-err",
        totalArquivos: 3,
        totalDiretorios: 1,
        totalErros: 0,
        durationMs: 200,
      });
    });

    await waitFor(() => screen.getByRole("button", { name: /indexar arquivos/i }));
    fireEvent.click(screen.getByRole("button", { name: /indexar arquivos/i }));
    await waitFor(() => expect(indexingFailedCb).toBeDefined());

    act(() => { indexingFailedCb!(); });

    await waitFor(() =>
      expect(screen.getByText(/erro/i)).toBeInTheDocument(),
    );
  });
});
