import { useEffect, useState } from "react";

import type { UnlistenFn } from "@tauri-apps/api/event";

import { ThemeToggle } from "@/components/theme-toggle";
import { Analise } from "@/features/conhecimento/Analise";
import { Descoberta } from "@/features/descoberta/Descoberta";
import { useTranslations } from "@/i18n";
import { announceReady } from "@/ipc/commands";
import { onReady } from "@/ipc/events";
import { usePing } from "@/ipc/queries";

function App() {
  const t = useTranslations();
  const { data: version, isLoading, isError } = usePing();
  const [readyMessage, setReadyMessage] = useState<string | null>(null);

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;
    void (async () => {
      // Registra o listener antes de pedir a emissão, garantindo a entrega.
      unlisten = await onReady((payload) => setReadyMessage(payload.message));
      await announceReady();
    })();
    return () => unlisten?.();
  }, []);

  return (
    <main className="relative flex min-h-screen flex-col items-center justify-center gap-4 bg-background p-8 text-foreground">
      <div className="absolute right-4 top-4">
        <ThemeToggle />
      </div>

      <h1 className="text-2xl font-semibold">{t.app.title}</h1>

      <p className="text-muted-foreground">
        {isLoading
          ? t.app.loading
          : isError
            ? t.app.error
            : `${t.app.version}: ${version}`}
      </p>

      {readyMessage && (
        <p className="text-sm text-muted-foreground">{readyMessage}</p>
      )}

      <Descoberta />
      <Analise />
    </main>
  );
}

export default App;
