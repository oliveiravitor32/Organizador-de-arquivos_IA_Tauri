# ADR-019 — Similaridade Vetorial: Cosseno em Memória com ndarray

**Status:** Aceita  
**Data:** 2026-06-13  
**Marco:** M2 — Conhecimento

---

## Contexto

UC-011 (Construir Clusters) e UC-014 (Busca Semântica) requerem calcular proximidade semântica entre embeddings.

Duas estratégias foram consideradas:
- **Cosseno em memória (ndarray):** carrega vetores do SQLite e calcula similaridade em Rust puro.
- **sqlite-vss:** extensão SQLite que adiciona índices vetoriais ANN diretamente no banco.

---

## Decisão

Adotar **similaridade de cosseno calculada em memória** usando a crate `ndarray`.

Vetores são carregados do SQLite (como BLOB — ver ADR-020) e comparados em lote por operações matriciais.

---

## Motivações

- **Adequado ao volume local:** o caso de uso é organizador pessoal de arquivos, não busca em escala web. Até ~50k arquivos com vetores de 768 dimensões cabem confortavelmente em RAM.
- **Zero dependência nativa:** sqlite-vss requer compilar uma extensão C com problemas recorrentes no Windows; distribuir `.dll` no bundle Tauri adiciona complexidade de empacotamento.
- **Alinhamento com local-first (ADR-003):** nenhum serviço externo, nenhuma extensão de banco que precise estar presente no sistema do usuário.
- **ndarray já é dependência indireta** de vários crates Rust do ecossistema; adicionar explicitamente tem custo baixo.

---

## Consequências

### Positivas

- Implementação simples, testável, sem configuração de extensões.
- Funciona em qualquer ambiente Windows/Linux onde o app Tauri rodar.
- Migração futura para sqlite-vss ou usearch é transparente — basta trocar a implementação do repositório.

### Negativas

- Busca vetorial é O(n) por query (varredura completa dos embeddings). Aceitável até ~50k vetores.
- Carrega todos os vetores em RAM para comparação em lote.

---

## Limiar de Migração

Se o número de arquivos indexados regularmente exceder ~50k, reavaliar para sqlite-vss ou usearch em M6.

---

## Alternativa Rejeitada

**sqlite-vss:** problemas de compilação e distribuição no Windows tornam o custo de integração alto demais para o benefício de performance em volumes locais.
