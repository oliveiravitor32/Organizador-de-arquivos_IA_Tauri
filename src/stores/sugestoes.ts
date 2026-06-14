import { create } from "zustand";

import type { SugestaoItem } from "@/ipc/commands";

export type SugestoesStatus = "idle" | "generating" | "done" | "error";

interface SugestoesState {
  status: SugestoesStatus;
  generationId: string | null;
  sugestoes: SugestaoItem[];
  stats: { geradas: number; descartadas: number; durationMs: number } | null;
  erro: string | null;

  setStarted: (generationId: string, total: number) => void;
  addSugestao: (sugestao: SugestaoItem) => void;
  setCompleted: (stats: { geradas: number; descartadas: number; durationMs: number }) => void;
  setError: (msg: string) => void;
  reset: () => void;
}

export const useSugestoesStore = create<SugestoesState>((set) => ({
  status: "idle",
  generationId: null,
  sugestoes: [],
  stats: null,
  erro: null,

  setStarted: (generationId) =>
    set({ status: "generating", generationId, sugestoes: [], stats: null, erro: null }),

  addSugestao: (sugestao) =>
    set((s) => ({ sugestoes: [...s.sugestoes, sugestao] })),

  setCompleted: (stats) => set({ status: "done", stats }),

  setError: (msg) => set({ status: "error", erro: msg }),

  reset: () =>
    set({ status: "idle", generationId: null, sugestoes: [], stats: null, erro: null }),
}));
