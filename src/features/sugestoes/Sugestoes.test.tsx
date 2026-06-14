import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor, act } from "@testing-library/react";

vi.mock("@/ipc/commands", () => ({
  gerarSugestoes: vi.fn(),
  explicarSugestao: vi.fn(),
}));

vi.mock("@/ipc/events", () => ({
  onSuggestionStarted: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionCreated: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionCompleted: vi.fn(() => Promise.resolve(() => {})),
  onSuggestionFailed: vi.fn(() => Promise.resolve(() => {})),
}));

import { gerarSugestoes, explicarSugestao } from "@/ipc/commands";
import {
  onSuggestionCompleted,
  onSuggestionCreated,
  onSuggestionFailed,
} from "@/ipc/events";
import { useSugestoesStore } from "@/stores/sugestoes";
import { Sugestoes } from "./Sugestoes";

function renderSugestoes() {
  return render(<Sugestoes />);
}

describe("Sugestoes — estado inicial", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useSugestoesStore.getState().reset();
  });

  it("exibe botão de gerar sugestões", () => {
    renderSugestoes();
    expect(screen.getByRole("button", { name: /gerar sugestões/i })).toBeInTheDocument();
  });

  it("exibe mensagem de vazio no estado idle", () => {
    renderSugestoes();
    expect(screen.getByTestId("vazio")).toBeInTheDocument();
  });
});

describe("Sugestoes — UC-005 CA-001: gerar sugestões", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useSugestoesStore.getState().reset();
  });

  it("chama gerarSugestoes ao clicar no botão", async () => {
    vi.mocked(gerarSugestoes).mockResolvedValue({ suggestionGenerationId: "g1" });
    renderSugestoes();
    fireEvent.click(screen.getByRole("button", { name: /gerar sugestões/i }));
    await waitFor(() => expect(gerarSugestoes).toHaveBeenCalledTimes(1));
  });

  it("exibe sugestão na lista após onSuggestionCreated", async () => {
    renderSugestoes();
    const createdCb = vi.mocked(onSuggestionCreated).mock.calls[0]?.[0];
    await act(async () => {
      createdCb?.({ suggestionId: "s1", titulo: "Grupo Fiscal", confianca: 0.9 });
    });
    expect(screen.getByTestId("lista-sugestoes")).toBeInTheDocument();
    expect(screen.getByText("Grupo Fiscal")).toBeInTheDocument();
  });

  it("UC-005 CA-003: exibe badge de confiança", async () => {
    renderSugestoes();
    const createdCb = vi.mocked(onSuggestionCreated).mock.calls[0]?.[0];
    await act(async () => {
      createdCb?.({ suggestionId: "s2", titulo: "Grupo X", confianca: 0.9 });
    });
    expect(screen.getByText(/alta confiança/i)).toBeInTheDocument();
  });

  it("UC-005 CA-004: clicar em sugestão abre detalhe", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue({
      suggestionId: "s3",
      tipo: "agrupamento",
      titulo: "Grupo Y",
      justificativa: "3 arquivos similares",
      evidencias: [],
      confianca: 0.8,
      desatualizada: false,
      operacoes: 1,
      arquivos: [],
    });
    renderSugestoes();
    const createdCb = vi.mocked(onSuggestionCreated).mock.calls[0]?.[0];
    await act(async () => {
      createdCb?.({ suggestionId: "s3", titulo: "Grupo Y", confianca: 0.8 });
    });
    fireEvent.click(screen.getByTestId("sugestao-s3"));
    await waitFor(() => expect(screen.getByTestId("detalhe-sugestao")).toBeInTheDocument());
  });

  it("exibe erro quando gerarSugestoes rejeita", async () => {
    vi.mocked(gerarSugestoes).mockRejectedValue(new Error("Ollama offline"));
    renderSugestoes();
    fireEvent.click(screen.getByRole("button", { name: /gerar sugestões/i }));
    await waitFor(() =>
      expect(screen.getByRole("alert")).toHaveTextContent(/Ollama offline/i),
    );
  });

  it("exibe erro quando onSuggestionFailed dispara", async () => {
    renderSugestoes();
    const failedCb = vi.mocked(onSuggestionFailed).mock.calls[0]?.[0];
    await act(async () => {
      failedCb?.({ suggestionGenerationId: "g1", error: "Falha interna" });
    });
    expect(screen.getByRole("alert")).toHaveTextContent(/Falha interna/i);
  });

  it("exibe stats após onSuggestionCompleted", async () => {
    renderSugestoes();
    const completedCb = vi.mocked(onSuggestionCompleted).mock.calls[0]?.[0];
    await act(async () => {
      completedCb?.({
        suggestionGenerationId: "g1",
        stats: { geradas: 3, descartadas: 1, durationMs: 400 },
      });
    });
    expect(screen.getByText(/sugestões geradas/i)).toBeInTheDocument();
  });
});
