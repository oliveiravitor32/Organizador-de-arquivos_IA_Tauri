# ADR-024 — Limiar de Confiança Mínima para Sugestões

**Status:** Aceito  
**Data:** 2026-06-14  
**Marco:** M3 — Inteligência

---

## Contexto

UC-005 define que sugestões com confiança `< 0,50` devem ser descartadas, mas
não formaliza esse valor como decisão arquitetural.

No Marco 2, a confiança de um cluster é calculada como a média das
similaridades de cosseno entre todos os pares de arquivos membros (ADR-022).
Esse valor é reutilizado como confiança da sugestão derivada do cluster.

A escolha do limiar impacta diretamente:
- **Volume:** limiares baixos geram mais sugestões, incluindo agrupamentos fracos.
- **Precisão:** limiares altos geram menos sugestões, com maior coerência semântica.

---

## Decisão

O sistema descarta sugestões com `confiança < 0,50`, conforme especificado em
UC-005.

O valor `0,50` é formalizado como constante nomeada no código Rust:

```rust
pub const MIN_SUGGESTION_CONFIDENCE: f64 = 0.50;
```

---

## Alternativas consideradas

| Limiar | Característica | Motivo da rejeição |
|---|---|---|
| `0,50` | Equilíbrio volume/qualidade | **Escolhido** — alinhado à spec |
| `0,60` | Mais restritivo | Reduziria sugestões sem base documentada |
| `0,75` | Mesmo threshold do clustering | Redundante: só clusters de altíssima coerência seriam exibidos |

---

## Consequências

- Clusters com confiança entre `0,50` e `0,74` geram sugestões de
  **baixa confiança** (exibidas com aviso visual na UI).
- Clusters com confiança entre `0,75` e `1,00` geram sugestões de
  **alta/média confiança**.
- Clusters abaixo de `0,50` são silenciosamente descartados (sem sugestão gerada).
- O limiar é configurável por constante — pode ser exposto como preferência do
  usuário em Marco 6 (ADR-013).

---

## Referências

- UC-005 — Gerar Sugestões (faixas de confiança)
- ADR-022 — Clusterização por threshold cosseno (confiança de cluster)
- ADR-013 — Configuração e Observabilidade
