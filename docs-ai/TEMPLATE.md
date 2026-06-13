# TEMPLATE — Fluxo de Feature

Copie este molde para `docs-ai/features/<nome-da-feature>/`, dividindo nos três arquivos abaixo.

Preencha em ordem. Cada etapa deriva da anterior.

---

# 1-pesquisa.md

## Feature

Nome e objetivo em uma frase.

## Casos de uso envolvidos

Liste os UCs do `docs/` relevantes (por ID e link).

## Decisões aplicáveis

ADRs que restringem ou orientam esta feature.

## Contratos e dados

- Commands relevantes (`docs/arquitetura/contratos-tauri.md`)
- Events relevantes (`docs/arquitetura/catalogo-de-eventos.md`)
- Tabelas/esquema (`docs/arquitetura/esquema-sql.md`)

## Onde mora o código

Módulos afetados (`docs/arquitetura/estrutura-do-projeto.md`).

## Critérios de aceitação

Os CAs dos UCs que esta feature precisa satisfazer.

## Riscos e questões em aberto

O que não está claro no `docs/` e precisa de decisão.

---

# 2-planejamento.md

## Abordagem

Estratégia técnica em alto nível, derivada da pesquisa.

## Tarefas

Lista ordenada, com dependências explícitas.

## Pontos de integração

Onde esta feature toca outras partes do sistema.

## Plano de testes

Como cada CA vira teste (`docs/requisitos/estrategia-de-testes.md`).

## Decisões tomadas

Escolhas feitas durante o planejamento (e por quê).

---

# 3-execucao.md

## Checklist de implementação

Passos concretos, em ordem de execução, marcáveis.

## Definition of Done

- [ ] CAs cobertos por teste
- [ ] Contratos (commands/events) respeitados
- [ ] Sem violação das regras inegociáveis (CLAUDE.md)
- [ ] UI usa só tokens de tema (sem cor crua / classe de cor direta do Tailwind)
- [ ] Suíte passa offline e determinística

## Verificação

Como provar que funciona (comandos, fluxo manual, testes).

## Registro

O que foi efetivamente feito e qualquer desvio do plano.
