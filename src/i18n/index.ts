import { ptBR } from "./pt-BR";

/**
 * Estrutura i18n-ready: o MVP é pt-BR, mas as mensagens ficam centralizadas
 * para permitir adicionar idiomas no futuro sem refatorar componentes.
 */
export type Messages = typeof ptBR;

const messages: Messages = ptBR;

/** Retorna o catálogo de mensagens do idioma ativo. */
export function useTranslations(): Messages {
  return messages;
}
