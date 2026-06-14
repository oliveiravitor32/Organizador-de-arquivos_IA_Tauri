import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";

vi.mock("@/ipc/commands", () => ({
  explicarSugestao: vi.fn(),
}));

import { explicarSugestao } from "@/ipc/commands";
import { DetalhesSugestao } from "./DetalhesSugestao";

const mockBase = {
  suggestionId: "s1",
  tipo: "agrupamento",
  titulo: "Grupo Fiscal",
  justificativa: "4 arquivos com similaridade 0.87 distribuídos em 3 diretórios.",
  evidencias: [
    { tipo: "arquivos_no_cluster", valor: "4" },
    { tipo: "similaridade_media", valor: "0.87" },
    { tipo: "diretorios_distintos", valor: "3" },
  ],
  confianca: 0.87,
  desatualizada: false,
  operacoes: 1,
  arquivos: [
    { id: "f1", nome: "contrato.pdf", caminho: "/docs/contrato.pdf" },
    { id: "f2", nome: "cronograma.xlsx", caminho: "/projetos/cronograma.xlsx" },
  ],
};

function renderDetalhe(props?: Partial<Parameters<typeof DetalhesSugestao>[0]>) {
  return render(<DetalhesSugestao suggestionId="s1" onFechar={vi.fn()} {...props} />);
}

describe("DetalhesSugestao — UC-012 CA-001: justificativa", () => {
  beforeEach(() => vi.clearAllMocks());

  it("exibe justificativa após carregar", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue(mockBase);
    renderDetalhe();
    await waitFor(() =>
      expect(screen.getByTestId("justificativa")).toHaveTextContent(
        /4 arquivos com similaridade/i,
      ),
    );
  });
});

describe("DetalhesSugestao — UC-012 CA-002: evidências rastreáveis", () => {
  beforeEach(() => vi.clearAllMocks());

  it("exibe lista de evidências com tipo e valor", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue(mockBase);
    renderDetalhe();
    await waitFor(() => expect(screen.getByTestId("lista-evidencias")).toBeInTheDocument());
    expect(screen.getByText(/arquivos_no_cluster: 4/i)).toBeInTheDocument();
    expect(screen.getByText(/diretorios_distintos: 3/i)).toBeInTheDocument();
  });
});

describe("DetalhesSugestao — UC-012 CA-003: confiança exibida", () => {
  beforeEach(() => vi.clearAllMocks());

  it("exibe valor de confiança como porcentagem", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue(mockBase);
    renderDetalhe();
    await waitFor(() =>
      expect(screen.getByTestId("confianca-valor")).toHaveTextContent("87%"),
    );
  });
});

describe("DetalhesSugestao — arquivos do grupo (sempre visíveis)", () => {
  beforeEach(() => vi.clearAllMocks());

  it("exibe lista de arquivos do cluster", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue(mockBase);
    renderDetalhe();
    await waitFor(() => expect(screen.getByTestId("lista-arquivos")).toBeInTheDocument());
    expect(screen.getByText("contrato.pdf")).toBeInTheDocument();
    expect(screen.getByText("cronograma.xlsx")).toBeInTheDocument();
  });

  it("não exibe seção quando não há arquivos", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue({ ...mockBase, arquivos: [] });
    renderDetalhe();
    await waitFor(() => expect(screen.getByTestId("justificativa")).toBeInTheDocument());
    expect(screen.queryByTestId("lista-arquivos")).not.toBeInTheDocument();
  });
});

describe("DetalhesSugestao — UC-012 CA-004: sinaliza desatualizada", () => {
  beforeEach(() => vi.clearAllMocks());

  it("exibe badge desatualizada quando flag é true", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue({ ...mockBase, desatualizada: true });
    renderDetalhe();
    await waitFor(() =>
      expect(screen.getByTestId("badge-desatualizada")).toBeInTheDocument(),
    );
  });

  it("não exibe badge quando desatualizada é false", async () => {
    vi.mocked(explicarSugestao).mockResolvedValue(mockBase);
    renderDetalhe();
    await waitFor(() => expect(screen.getByTestId("justificativa")).toBeInTheDocument());
    expect(screen.queryByTestId("badge-desatualizada")).not.toBeInTheDocument();
  });
});
