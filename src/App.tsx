import { useEffect, useState } from "react";

import { ThemeToggle } from "@/components/theme-toggle";
import { useTranslations } from "@/i18n";
import { onReady } from "@/ipc/events";
import { usePing } from "@/ipc/queries";

function App() {
  const t = useTranslations();
  const { data: version, isLoading, isError } = usePing();
  const [readyMessage, setReadyMessage] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = onReady((payload) => setReadyMessage(payload.message));
    return () => {
      unlisten.then((fn) => fn());
    };
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
    </main>
  );
}

export default App;
