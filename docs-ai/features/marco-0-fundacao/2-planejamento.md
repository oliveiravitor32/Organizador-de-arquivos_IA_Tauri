# 2-planejamento.md — Marco 0: Fundação

Derivado de `1-pesquisa.md`.

## Abordagem

Montar a fundação de baixo para cima e validar cada camada com um "fio condutor" mínimo (walking skeleton): banco → backend → fronteira → frontend → testes. Ao final, um clique no frontend chama um command que lê o banco e dispara um evento — provando que toda a pilha funciona ponta a ponta, sem nenhuma lógica de domínio.

## Decisões tomadas

Resolvendo as questões em aberto da pesquisa:

- **Tauri v2** — API atual de commands/events e melhor suporte. (registrar como ADR-011 se o projeto quiser formalizar)
- **`sqlx` com SQLite** — migrações versionadas em arquivos `.sql`, checagem em tempo de compilação. Alternativa `rusqlite` rejeitada por migração manual.
- **Vite** — bundler padrão do template React do Tauri.
- **Migração única inicial** (`0001_init.sql`) contendo todo o esquema de `esquema-sql.md`.

> Nota: estas decisões de ferramenta foram formalizadas em [ADR-011](../../../docs/decisoes/ADR-011-tauri-v2-sqlx-vite.md), mantendo o princípio "nenhuma decisão fora dos ADRs".

## Tarefas

1. **Scaffold Tauri v2 + React + Vite** — gerar projeto base; app abre janela vazia. *(sem dependências)*
2. **Ajustar estrutura de pastas** ao `estrutura-do-projeto.md` (criar `commands/`, `events/`, `core/`, `services/`, `db/`, `domain/`, `error.rs`; e `src/ipc/`). *(dep: 1)*
3. **Configurar `sqlx` + conexão SQLite** com `foreign_keys=ON` e `WAL`; estado compartilhado em `core/state.rs`. *(dep: 2)*
4. **Migração `0001_init.sql`** com todas as tabelas e índices de `esquema-sql.md`; aplicar na inicialização; versionar via `user_version`. *(dep: 3)*
5. **Tipo de erro estruturado** (`error.rs`) com `code`, `message`, `details`, serializável ao frontend. *(dep: 2)*
6. **Command de teste `ping`** retornando versão do app; registrar na fronteira. *(dep: 2, 5)*
7. **Evento de teste** emitido pelo backend e recebido no frontend; wrappers em `src/ipc/`. *(dep: 6)*
8. **Base de UI** — instalar Tailwind + shadcn/ui; `styles/globals.css` com os tokens dos temas claro/escuro (preto suave / neutro claro); alternador de tema com persistência; `components/ui/` criado. *(dep: 2, ADR-012, frontend-ui.md)*
9. **Tela inicial vazia** que chama `ping` e escuta o evento, exibindo o resultado, já respeitando tokens e estados. *(dep: 7, 8)*
10. **Suíte de testes base:** 1 unidade (erro estruturado), 1 integração (migração cria tabelas em SQLite efêmero), estrutura `tests/e2e` pronta. *(dep: 4, 5)*

## Pontos de integração

- `src/ipc/` é o único ponto onde o frontend fala com o backend — define o padrão para todos os marcos seguintes.
- A migração inicial é o contrato físico que todos os repositórios futuros assumirão.
- `core/state.rs` carregará, nos próximos marcos, filas e o serviço de IA.

## Plano de testes

Conforme `estrategia-de-testes.md`:

| CA | Teste |
| --- | --- |
| CA-2 | Integração: aplicar migração em SQLite temporário e verificar existência de todas as tabelas. |
| CA-3 | Contrato/e2e: invocar `ping` e validar retorno. |
| CA-4 | Contrato/e2e: registrar listener e confirmar recebimento do evento. |
| CA-5 | Suíte roda offline; sem dependência de IA (dublês ainda não necessários neste marco). |

CA-1 é validado manualmente (app abre).

## Saída esperada para a execução

Lista de tarefas 1–9 com dependências resolvidas, pronta para virar checklist em `3-execucao.md`.
