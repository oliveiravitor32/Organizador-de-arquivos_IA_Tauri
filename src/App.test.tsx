import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, act, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import type { ReadyPayload } from "@/ipc/events";

// Ordem das chamadas e captura do callback do listener.
const calls: string[] = [];
let readyCallback: ((payload: ReadyPayload) => void) | undefined;

vi.mock("@/ipc/events", () => ({
  onReady: vi.fn((cb: (payload: ReadyPayload) => void) => {
    calls.push("listen");
    readyCallback = cb;
    return Promise.resolve(() => {});
  }),
  // Marco 1 — listeners usados por Descoberta.tsx (montado dentro de App).
  onScanProgress: vi.fn(() => Promise.resolve(() => {})),
  onScanCompleted: vi.fn(() => Promise.resolve(() => {})),
  onScanFailed: vi.fn(() => Promise.resolve(() => {})),
  onScanCancelled: vi.fn(() => Promise.resolve(() => {})),
  onIndexingProgress: vi.fn(() => Promise.resolve(() => {})),
  onIndexingCompleted: vi.fn(() => Promise.resolve(() => {})),
  onIndexingFailed: vi.fn(() => Promise.resolve(() => {})),
  // Marco 2 — listeners usados por Analise.tsx
  onAnalysisStarted: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisProgress: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisCompleted: vi.fn(() => Promise.resolve(() => {})),
  onAnalysisFailed: vi.fn(() => Promise.resolve(() => {})),
  // Marco 3 — listeners usados por Sugestoes.tsx
  onSuggestionStarted: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionCreated: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionCompleted: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionFailed: vi.fn(() => Promise.resolve(() => {})),
}));

vi.mock("@/ipc/commands", () => ({
  ping: vi.fn(() => Promise.resolve("0.1.0")),
  announceReady: vi.fn(() => {
    calls.push("announce");
    return Promise.resolve();
  }),
  escanearDiretorio: vi.fn(),
  indexarArquivos: vi.fn(),
  cancelarOperacao: vi.fn(),
  consultarIndexacao: vi.fn(() => Promise.resolve(null)),
  analisarArquivos: vi.fn(),
  gerarSugestoes: vi.fn(),
  explicarSugestao: vi.fn(),
  listarSugestoes: vi.fn(),
}));

import App from "./App";

function renderApp() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return render(
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>,
  );
}

describe("App — regressão CA-4 (evento de prontidão)", () => {
  beforeEach(() => {
    calls.length = 0;
    readyCallback = undefined;
  });

  it("registra o listener antes de chamar announce_ready", async () => {
    renderApp();

    // Sem a correção, announce_ready nunca era chamado: o waitFor falharia.
    await waitFor(() => expect(calls).toContain("announce"));

    expect(calls.indexOf("listen")).toBeGreaterThanOrEqual(0);
    expect(calls.indexOf("listen")).toBeLessThan(calls.indexOf("announce"));
  });

  it("exibe a mensagem de prontidão quando o evento chega", async () => {
    renderApp();

    await waitFor(() => expect(readyCallback).toBeDefined());

    act(() => {
      readyCallback!({ message: "Backend pronto" });
    });

    expect(await screen.findByText("Backend pronto")).toBeInTheDocument();
  });
});
