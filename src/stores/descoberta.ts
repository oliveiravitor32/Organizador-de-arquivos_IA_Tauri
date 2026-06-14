import { create } from "zustand";

import type {
  IndexingCompletedPayload,
  ScanCompletedPayload,
} from "@/ipc/events";

export type OperacaoStatus =
  | "idle"
  | "scanning"
  | "scan_done"
  | "indexing"
  | "indexing_done"
  | "cancelled"
  | "error";

interface DescobertaState {
  status: OperacaoStatus;
  scanId: string | null;
  indexingId: string | null;
  scanStats: ScanCompletedPayload | null;
  indexingStats: IndexingCompletedPayload | null;
  scanProgress: { filesFound: number; dirsFound: number } | null;
  indexingProgress: { processed: number; total: number } | null;
  erro: string | null;

  setScanStarted: (scanId: string) => void;
  setScanProgress: (filesFound: number, dirsFound: number) => void;
  setScanCompleted: (stats: ScanCompletedPayload) => void;
  setIndexingStarted: (indexingId: string) => void;
  setIndexingProgress: (processed: number, total: number) => void;
  setIndexingCompleted: (stats: IndexingCompletedPayload) => void;
  setError: (msg: string) => void;
  setCancelled: () => void;
  reset: () => void;
}

export const useDescobertaStore = create<DescobertaState>((set) => ({
  status: "idle",
  scanId: null,
  indexingId: null,
  scanStats: null,
  indexingStats: null,
  scanProgress: null,
  indexingProgress: null,
  erro: null,

  setScanStarted: (scanId) =>
    set({ status: "scanning", scanId, scanProgress: null, erro: null }),

  setScanProgress: (filesFound, dirsFound) =>
    set({ scanProgress: { filesFound, dirsFound } }),

  setScanCompleted: (stats) =>
    set({ status: "scan_done", scanStats: stats }),

  setIndexingStarted: (indexingId) => {
    localStorage.setItem("activeIndexingId", indexingId);
    set({ status: "indexing", indexingId, indexingProgress: null });
  },

  setIndexingProgress: (processed, total) =>
    set({ indexingProgress: { processed, total } }),

  setIndexingCompleted: (stats) => {
    localStorage.removeItem("activeIndexingId");
    set({ status: "indexing_done", indexingStats: stats });
  },

  setError: (msg) => set({ status: "error", erro: msg }),

  setCancelled: () => set({ status: "cancelled" }),

  reset: () => {
    localStorage.removeItem("activeIndexingId");
    set({
      status: "idle",
      scanId: null,
      indexingId: null,
      scanStats: null,
      indexingStats: null,
      scanProgress: null,
      indexingProgress: null,
      erro: null,
    });
  },
}));
