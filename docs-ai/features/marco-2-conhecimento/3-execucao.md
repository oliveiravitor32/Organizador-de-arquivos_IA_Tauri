# 3-execucao — Marco 2: Conhecimento

## Checklist de implementação

### T1 — Dependências e Migração
- [ ] Adicionar `ndarray`, `bytemuck` ao `Cargo.toml`
- [ ] Criar `migrations/0003_marco2_conhecimento.sql` (índices faltantes)

### T2 — Cliente Ollama + Trait ServicoIa
- [ ] `services/ia/mod.rs` — trait `ServicoIa` + tipos `EntidadeExtraida`, `RelacaoInferida`
- [ ] `services/ia/ollama/health.rs` — verificação de modelos disponíveis
- [ ] `services/ia/ollama/generate.rs` — POST /api/generate com JSON mode
- [ ] `services/ia/ollama/embed.rs` — POST /api/embed + serialização BLOB f32 LE
- [ ] `services/ia/ollama/mod.rs` — struct `OllamaService` implementa `ServicoIa`
- [ ] Testes unitários do cliente (mocks de resposta HTTP)

### T3 — Repositórios
- [ ] `db/repositories/entities.rs` — upsert_entity, link_file_entity, find_entities_by_file
- [ ] `db/repositories/embeddings.rs` — upsert_embedding, find_all_embeddings, find_embedding_by_file
- [ ] `db/repositories/relationships.rs` — upsert_relationship, find_relationships_by_file
- [ ] `db/repositories/clusters.rs` — upsert_cluster, add_cluster_member, find_clusters
- [ ] `db/repositories/mod.rs` — expor novos repos
- [ ] `domain/conhecimento.rs` — Entity, Embedding, Relation, Cluster
- [ ] Testes de integração para todos os métodos dos repos

### T4 — Serviços de IA
- [ ] `services/conhecimento/entidades.rs` — ExtracaoEntidadesService
- [ ] `services/conhecimento/embeddings.rs` — EmbeddingService (com chunking)
- [ ] `services/conhecimento/relacoes.rs` — RelacoesService (filtro de confiança)
- [ ] `services/conhecimento/clusters.rs` — ClusterService (matriz de cosseno + threshold)
- [ ] `services/conhecimento/grafo.rs` — GrafoService (consolida MENCIONA, SIMILAR_A, PERTENCE_A)
- [ ] `services/conhecimento/mod.rs` — expor serviços
- [ ] Testes unitários de cada serviço com `MockServicoIa`

### T5 — AnaliseService + Command
- [ ] `services/conhecimento/analise.rs` — AnaliseService (orquestrador UC-003)
- [ ] `commands/conhecimento.rs` — `analisar_arquivos`
- [ ] `commands/mod.rs` — expor `conhecimento`
- [ ] `lib.rs` — registrar command + módulos
- [ ] Teste de integração: pipeline completo com arquivos reais em disco

### T6 — Frontend
- [ ] `stores/analise.ts` + `stores/analise.test.ts`
- [ ] `ipc/commands.ts` — adicionar `analisarArquivos`
- [ ] `ipc/events.ts` — adicionar listeners `analysis://`
- [ ] `i18n/pt-BR.ts` — namespace `analise`
- [ ] `features/conhecimento/Analise.tsx` + `Analise.test.tsx`
- [ ] `App.tsx` — integrar `<Analise />` (visível após `indexing_done`)
- [ ] `App.test.tsx` — adicionar mocks dos novos eventos

---

## Definition of Done

- [ ] Cada CA listado no 2-planejamento tem ≥ 1 teste correspondente
- [ ] Testes unitários seguem `convencoes-de-teste.md` (inline `#[cfg(test)]`, ao lado do arquivo)
- [ ] `MockServicoIa` usado em todos os testes de serviço — Ollama nunca chamado em testes
- [ ] Cobertura reportada (`cargo test` + `npm test`)
- [ ] Commands `analisar_arquivos` respeita contratos de `contratos-tauri.md`
- [ ] Events `analysis://` respeitam `catalogo-de-eventos.md`
- [ ] Sem violação das regras inegociáveis (local-first, sem move de arquivo, aprovação obrigatória não se aplica aqui — apenas leitura)
- [ ] UI usa só tokens semânticos (`text-foreground`, `bg-background`, etc.)
- [ ] Ollama ausente não trava a UI — erro estruturado exibido
- [ ] Suíte completa passa offline (testes não fazem chamadas reais ao Ollama)

---

## Verificação

```bash
# Backend — todos os testes
cd src-tauri && cargo test

# Frontend — todos os testes
npm test -- --run

# Smoke test manual:
# 1. Garantir que Ollama esteja rodando:
#      ollama serve
#      ollama pull qwen3:4b
#      ollama pull nomic-embed-text
# 2. Iniciar app: npm run tauri dev
# 3. Selecionar diretório → Escanear → Indexar → Analisar
# 4. Verificar no banco:
#      SELECT COUNT(*) FROM entities;
#      SELECT COUNT(*) FROM embeddings;
#      SELECT COUNT(*) FROM clusters;
```

---

## Registro

*(Preenchido durante e após a implementação)*

| Item | Status | Observação |
| --- | --- | --- |
| T1 — Dependências | pendente | |
| T2 — Cliente Ollama | pendente | |
| T3 — Repositórios | pendente | |
| T4 — Serviços de IA | pendente | |
| T5 — AnaliseService | pendente | |
| T6 — Frontend | pendente | |
