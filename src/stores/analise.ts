import { create } from "zustand";

import type { AnalysisCompletedPayload } from "@/ipc/events";

export type AnaliseStatus = "idle" | "analysing" | "done" | "error";

interface AnaliseState {
  status: AnaliseStatus;
  analysisId: string | null;
  progress: { processed: number; total: number; currentFile: string } | null;
  stats: AnalysisCompletedPayload["stats"] | null;
  erro: string | null;

  setStarted: (analysisId: string, total: number) => void;
  setProgress: (processed: number, total: number, currentFile: string) => void;
  setCompleted: (stats: AnalysisCompletedPayload["stats"]) => void;
  setError: (msg: string) => void;
  reset: () => void;
}

export const useAnaliseStore = create<AnaliseState>((set) => ({
  status: "idle",
  analysisId: null,
  progress: null,
  stats: null,
  erro: null,

  setStarted: (analysisId, total) =>
    set({ status: "analysing", analysisId, progress: { processed: 0, total, currentFile: "" }, erro: null }),

  setProgress: (processed, total, currentFile) =>
    set({ progress: { processed, total, currentFile } }),

  setCompleted: (stats) => set({ status: "done", stats }),

  setError: (msg) => set({ status: "error", erro: msg }),

  reset: () =>
    set({ status: "idle", analysisId: null, progress: null, stats: null, erro: null }),
}));
