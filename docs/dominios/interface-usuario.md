# Domínio: Interface do Usuário

## Objetivo

Este documento define as telas, fluxos de navegação e estados da interface.

Ele descreve **o que** o usuário vê e faz, não **como** os componentes são implementados.

A interface é uma camada de apresentação: nenhuma regra de negócio reside nela (ver `tauri.md`).

---

# Filosofia

## Usuário no Controle

A interface nunca aplica alterações sem aprovação explícita (ver ADR-007).

---

## Transparência

Toda sugestão é apresentada com justificativa, evidências e confiança (ver ADR-009).

---

## Progresso Visível

Operações longas exibem progresso em tempo real, alimentado por eventos.

---

## Reversibilidade Acessível

A opção de desfazer é sempre alcançável a partir do histórico.

---

# Mapa de Telas

```text
Início
├── Indexação
├── Sugestões
│   └── Detalhe da Sugestão (explicação)
├── Execução
├── Histórico
│   └── Detalhe da Execução (desfazer)
└── Exploração
    ├── Busca Semântica
    └── Contexto do Grafo
```

---

# Tela: Início

## Objetivo

Selecionar o diretório raiz e acompanhar o estado geral.

## Ações

- selecionar diretório
- iniciar escaneamento

## Estados

- vazio (nenhum diretório)
- escaneando
- pronto

**Commands:** escanear_diretorio
**Eventos:** ScanProgress, ScanCompleted

---

# Tela: Indexação

## Objetivo

Acompanhar a indexação e a análise dos arquivos.

## Ações

- iniciar indexação
- iniciar análise
- cancelar operação

## Estados

- descoberto
- indexando
- analisando
- concluído
- falha parcial

**Commands:** indexar_arquivos, analisar_arquivos, cancelar_operacao
**Eventos:** IndexingProgress, AnalysisStarted, AnalysisCompleted, AnalysisFailed

---

# Tela: Sugestões

## Objetivo

Revisar as sugestões geradas (UC-013).

## Ações

- aprovar
- rejeitar
- ajustar
- adiar
- abrir detalhe

## Estados

- carregando
- lista com sugestões pendentes
- vazia

**Commands:** listar_sugestoes, aprovar_sugestao, rejeitar_sugestao, ajustar_sugestao
**Eventos:** SuggestionApproved, SuggestionRejected, SuggestionAdjusted

---

# Tela: Detalhe da Sugestão

## Objetivo

Apresentar a explicação completa de uma sugestão (UC-012).

## Conteúdo

- justificativa
- evidências rastreáveis
- nível de confiança
- aviso de desatualização quando aplicável

**Commands:** explicar_sugestao
**Eventos:** ExplanationGenerated, ExplanationIncomplete

---

# Tela: Execução

## Objetivo

Aplicar as sugestões aprovadas (UC-006).

## Ações

- revisar plano de execução
- confirmar execução
- cancelar

## Estados

- plano pronto
- criando snapshot
- executando
- concluído
- falha

**Commands:** aplicar_alteracoes, cancelar_operacao
**Eventos:** SnapshotCreated, ExecutionProgress, ExecutionCompleted, ExecutionFailed

---

# Tela: Histórico

## Objetivo

Listar execuções anteriores e permitir restauração (UC-007).

## Ações

- visualizar execução
- selecionar modo de restauração
- confirmar rollback

## Estados

- lista de execuções
- restaurando
- restaurado
- indisponível (snapshot ausente)

**Commands:** listar_execucoes, desfazer_alteracoes
**Eventos:** RollbackProgress, RollbackCompleted, RollbackFailed

---

# Tela: Exploração — Busca Semântica

## Objetivo

Encontrar arquivos por significado (UC-014).

## Ações

- informar consulta
- escolher modo (conteúdo, entidade, similaridade)
- abrir resultado

## Estados

- inicial
- buscando
- resultados
- sem resultados

**Commands:** buscar_semantica
**Eventos:** SearchCompleted, SearchEmpty

---

# Tela: Exploração — Contexto do Grafo

## Objetivo

Navegar pelas conexões de um elemento (UC-015).

## Ações

- selecionar ponto de partida
- expandir conexões
- navegar para elementos relacionados

## Estados

- nó selecionado
- expandindo
- nó isolado (sem conexões)

**Commands:** explorar_contexto
**Eventos:** NodeExpanded, ContextExplorationCompleted

---

# Fluxo Principal de Navegação

```text
Início
↓ (escanear + indexar + analisar)
Sugestões
↓ (revisar e aprovar)
Execução
↓ (aplicar)
Histórico
↘ (desfazer, quando necessário)

Exploração disponível a qualquer momento após a análise.
```

---

# Estados Globais de UI

## Carregando

Operação assíncrona em andamento.

---

## Erro

Falha estruturada recebida do backend, exibida de forma compreensível.

---

## Vazio

Ausência de dados, com orientação para o próximo passo.

---

# Acessibilidade e Feedback

- toda ação destrutiva exige confirmação
- todo processo longo exibe progresso
- toda falha apresenta mensagem clara e ação sugerida

---

# Observação

As telas consomem exclusivamente os commands e events definidos em `contratos-tauri.md` e `catalogo-de-eventos.md`.

A interface não possui acesso direto a banco, arquivos ou IA.
