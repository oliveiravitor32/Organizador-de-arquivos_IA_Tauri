# Arquitetura: Configuração e Segurança

## Objetivo

Este documento define como o sistema é configurado e quais garantias de segurança ele oferece sobre os dados e o sistema de arquivos do usuário.

Decisões de base em ADR-013 (configuração em TOML, logs locais) e ADR-003 (IA local / privacidade).

---

# Parte 1 — Configuração

## Formato e Localização

- Formato: **TOML**.
- Local: diretório de configuração da aplicação fornecido pelo sistema operacional (via API do Tauri).
- Um único arquivo de configuração por instalação.

---

## Conteúdo

```toml
[ia]
runtime = "ollama"        # runtime de IA
model = "qwen3:4b"        # modelo ativo (substituível — RNF-016)

[indexacao]
ignore = [".git", "node_modules", "*.tmp"]
ocr_enabled = false       # OCR em imagens (opcional)
max_threads = 4           # paralelismo (RNF-023)

[interface]
theme = "system"          # system | light | dark
```

Os campos refletem os pontos de configuração já previstos em `contratos-tauri.md` (commands `obter_configuracao` / `atualizar_configuracao`).

---

## Princípios

### Transparência

O arquivo é legível e editável pelo usuário.

---

### Validação ao Carregar

Toda configuração é validada na inicialização.

Valores inválidos ou ausentes usam padrões seguros; o sistema nunca falha silenciosamente por config malformada.

---

### Padrões Seguros

Na ausência de arquivo, o sistema cria um com valores padrão.

---

### Separação de Responsabilidades

Configuração (preferências) fica no TOML. Dados de domínio ficam no SQLite. Nunca se misturam.

---

# Parte 2 — Segurança

## Modelo de Confiança

O sistema opera com os arquivos do próprio usuário, na máquina do usuário. A segurança foca em **não causar dano** e **não vazar dados**, não em defesa contra terceiros.

---

## Escopo de Sistema de Arquivos

### Diretório Raiz

O sistema atua **somente dentro do diretório raiz** selecionado pelo usuário (ver `dominios/sistema-arquivos.md`).

---

### Validação de Caminho

Todo caminho é validado antes de qualquer operação:

- deve estar contido no diretório raiz
- traversal para fora do escopo (`..`) é rejeitado
- caminhos simbólicos que escapam do escopo são rejeitados

---

### Verificação de Permissões

Existência e permissões são verificadas antes de qualquer escrita.

---

## Operações Seguras

### Aprovação Obrigatória

Nenhuma alteração física sem confirmação explícita (ADR-007).

---

### Snapshot Antes de Alterar

Toda alteração exige snapshot prévio (ADR-010).

---

### Sem Exclusão Destrutiva

O sistema não exclui conteúdo de forma irreversível. Operações potencialmente destrutivas passam por quarentena (ADR-010).

---

## Privacidade

### Processamento Local

Nenhum conteúdo é enviado a serviços externos (ADR-003, RNF-002).

---

### Sem Telemetria

Nenhum dado de uso ou diagnóstico sai da máquina (ADR-013).

---

### Logs sem Conteúdo Sensível

Logs registram identificadores e metadados, nunca o conteúdo dos arquivos (ver `observabilidade.md`, RNF-003).

---

## Superfície de Comandos

O frontend só acessa o backend pelos commands registrados (`contratos-tauri.md`). Toda entrada é validada no Rust antes de produzir efeitos.

---

# Critérios de Aceitação

- CA-001: configuração é carregada de um TOML, com padrões seguros na ausência.
- CA-002: config inválida não derruba a aplicação.
- CA-003: operações fora do diretório raiz são rejeitadas.
- CA-004: nenhuma escrita ocorre sem validação de caminho e permissão.
- CA-005: nenhum dado sai da máquina (verificável: sem chamadas de rede em operação normal).
- CA-006: logs não contêm conteúdo de arquivos.

---

# Observação

Configuração e segurança são transversais a todos os marcos.

O Marco 0 estabelece o carregamento de configuração e a validação de caminho; os marcos seguintes herdam essas garantias.
