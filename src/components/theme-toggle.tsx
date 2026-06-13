import { Button } from "@/components/ui/button";
import { useThemeStore } from "@/stores/theme";
import { useTranslations } from "@/i18n";

/** Alterna entre tema claro e escuro. */
export function ThemeToggle() {
  const theme = useThemeStore((s) => s.theme);
  const toggle = useThemeStore((s) => s.toggle);
  const t = useTranslations();

  return (
    <Button
      variant="outline"
      size="icon"
      aria-label={t.app.toggleTheme}
      onClick={toggle}
    >
      {theme === "dark" ? "☀" : "☾"}
    </Button>
  );
}
