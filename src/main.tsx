import React from "react";
import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import App from "./App";
import { applyTheme, useThemeStore } from "@/stores/theme";
import "@/styles/globals.css";

// Aplica o tema inicial antes de renderizar.
applyTheme(useThemeStore.getState().theme);

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </React.StrictMode>,
);
