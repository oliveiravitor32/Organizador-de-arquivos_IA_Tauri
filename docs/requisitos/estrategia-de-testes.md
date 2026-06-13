# Requisitos: Estratégia de Testes

## Objetivo

Este documento define como a qualidade do sistema será verificada.

Seu propósito é transformar os **Critérios de Aceitação** dos casos de uso e os **requisitos não funcionais** em testes concretos e rastreáveis.

---

# Princípios

## Rastreabilidade

Todo Critério de Aceitação (CA) de um caso de uso deve ter ao menos um teste correspondente.

---

## Pirâmide de Testes

A maior parte da cobertura está na base (unidade), reduzindo em direção ao topo (e2e).

```text
        e2e
     integração
   unidade (base)
```

---

## Determinismo

Testes não dependem de IA real para serem determinísticos.

A inferência é substituída por dublês nos testes automatizados.

---

## Independência

Cada teste prepara e limpa seu próprio estado.

---

## Local-First

A suíte deve rodar inteiramente offline.

---

# Níveis de Teste

## Unidade

**Alvo:** funções e regras de negócio isoladas.

**Onde:** backend (Rust) e lógica de frontend (React).

Exemplos:

- cálculo da operação inversa (ADR-010)
- validação de conflito de nome
- faixas de confiança

---

## Integração

**Alvo:** serviços contra dependências reais controladas.

**Onde:** `tests/integracao`.

Exemplos:

- repositórios contra SQLite real
- pipeline de indexação sobre um diretório de teste
- persistência e reconstrução do grafo

---

## Contrato

**Alvo:** os commands e events da camada Tauri.

Garante que entrada, saída e erros respeitam `contratos-tauri.md` e que os eventos seguem `catalogo-de-eventos.md`.

---

## Ponta a Ponta (e2e)

**Alvo:** fluxos completos do usuário.

**Onde:** `tests/e2e`.

Exemplos:

- escanear → indexar → analisar → sugerir → revisar → aplicar
- aplicar → desfazer (rollback completo)

---

# Dublês de IA

Para determinismo, o Serviço de IA é substituído por implementações de teste:

## Mock

Retornos fixos previsíveis.

---

## Fixtures

Conjuntos de entidades, embeddings e relações pré-gravados.

---

## Contrato do Adaptador

O adaptador Ollama real é testado separadamente, fora da suíte determinística.

---

# Dados de Teste

## Diretório de Amostra

Um conjunto fixo de arquivos cobrindo os formatos suportados (TXT, MD, PDF, DOCX, XLSX, imagem).

---

## Banco Efêmero

Cada teste de integração usa um SQLite temporário, criado pelas migrações de `esquema-sql.md`.

---

# Mapeamento Critério → Teste

Cada caso de uso contribui com testes derivados de seus CAs.

| Caso de Uso | Foco dos Testes |
| --- | --- |
| UC-001 Escanear | descoberta, itens ignorados, cancelamento |
| UC-002 Indexar | metadados, hash, reprocessamento incremental |
| UC-003 Analisar | orquestração, falha parcial, status final |
| UC-008–011 | entidades, embeddings, relações, clusters |
| UC-004 Grafo | consolidação de nós, consistência |
| UC-005 Sugerir | geração, confiança, ausência de regras fixas |
| UC-012 Explicar | justificativa, evidências rastreáveis |
| UC-013 Revisar | aprovação, rejeição, ajuste, conflito |
| UC-006 Aplicar | snapshot prévio, conflito, falha, rollback disponível |
| UC-007 Desfazer | inversa em ordem inversa, modos de restauração |
| UC-014 Buscar | relevância, busca vazia |
| UC-015 Explorar | conexões, nó isolado |

---

# Cobertura de Requisitos Não Funcionais

## Desempenho

Indexação de grandes volumes dentro de limites aceitáveis.

---

## Resiliência

Falhas individuais não interrompem o processamento global.

---

## Reversibilidade

Toda execução deve ser reversível (ADR-010).

---

## Privacidade

Nenhum dado sai da máquina durante a suíte (ADR-003).

---

# Critérios de Pronto

Uma funcionalidade é considerada testada quando:

- todos os CAs do UC possuem teste
- os fluxos alternativos críticos estão cobertos
- os contratos de command e event são verificados
- a suíte passa offline e de forma determinística

---

# Convenções e Ferramentas

As convenções concretas de arquivos de teste (onde ficam, nomenclatura, mocks) e as ferramentas (Vitest, React Testing Library, mockall, cobertura) estão em `convencoes-de-teste.md` (ADR-014).

---

# Observação

Os Critérios de Aceitação dos casos de uso são a fonte da verdade para o comportamento esperado.

Este documento define como verificá-los, não os redefine.
