# ADR-013 — Configuração em Arquivo e Observabilidade Local

## Status

Aceito

---

## Data

2026-06-13

---

## Contexto

O sistema precisa persistir configurações do usuário (modelo de IA, itens ignorados, tema, OCR, limites de processamento) e fornecer observabilidade para diagnóstico (RNF-019).

Como o projeto é local-first e prioriza privacidade (ADR-003, RNF-003), o tratamento de configuração e de logs precisa respeitar esses princípios.

Faltavam decisões concretas sobre **onde** guardar configuração e **como** tratar logs e telemetria.

---

## Problema

- Onde e como armazenar a configuração do usuário?
- Como registrar logs e métricas sem comprometer privacidade?

---

## Decisão

### Configuração

Armazenar a configuração do usuário em um **arquivo TOML** no diretório de configuração da aplicação fornecido pelo sistema operacional.

### Observabilidade

Adotar **logs locais estruturados**, com níveis, gravados em arquivo no diretório de logs da aplicação.

**Nenhuma telemetria.** Nenhum dado de uso ou diagnóstico sai da máquina.

---

## Justificativa

### Arquivo TOML

- Legível e editável diretamente pelo usuário (transparência).
- Fácil de versionar e fazer backup.
- Separa configuração (preferências) de dados (SQLite), evitando misturar responsabilidades.
- Formato simples, bem suportado em Rust.

---

### Logs locais sem telemetria

- Coerente com local-first e privacidade (ADR-003, RNF-003).
- Suficiente para diagnóstico (RNF-019).
- Logs nunca contêm conteúdo sensível dos arquivos (apenas identificadores e metadados).

---

## Alternativas Consideradas

### Configuração no SQLite

#### Motivo da Rejeição

Mistura preferências com dados de domínio; menos transparente; não editável fora do app.

---

### Tauri Store plugin

#### Motivo da Rejeição

Formato menos transparente para o usuário; dependência adicional sem ganho relevante para este caso.

---

### Telemetria opt-in

#### Motivo da Rejeição

Adiciona complexidade e superfície de risco de privacidade sem necessidade no MVP. Pode ser reconsiderada no futuro.

---

## Consequências

### Positivas

- Configuração transparente e portátil.
- Privacidade preservada por construção.
- Diagnóstico viável apenas com recursos locais.

---

### Negativas

- Sem telemetria, a detecção de problemas depende de o usuário compartilhar logs manualmente.
- Edição manual do TOML exige validação robusta ao carregar.

---

## Impacto

- **configuracao-e-seguranca.md** — detalha formato, localização e validação do arquivo.
- **observabilidade.md** — detalha níveis de log, o que registrar e o que nunca registrar.
- **Marco 0** — pode já criar o carregamento de config e o logger base.

---

## Decisão Final

Configuração em **arquivo TOML** no diretório de config do SO; **logs locais estruturados sem telemetria**. Ambos alinhados ao local-first e à privacidade.
