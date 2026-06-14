# ADR-022 — Clusterização: Threshold de Similaridade de Cosseno

**Status:** Aceita  
**Data:** 2026-06-13  
**Marco:** M2 — Conhecimento

---

## Contexto

UC-011 (Construir Clusters) precisa "identificar agrupamentos naturais" a partir de embeddings, entidades e relações.

Dois algoritmos foram considerados:
- **Threshold de cosseno simples:** agrupa elementos com similaridade acima de um limiar configurável.
- **DBSCAN:** algoritmo de clusterização por densidade, descobre automaticamente o número de clusters.

---

## Decisão

Adotar **threshold de similaridade de cosseno** como algoritmo de clusterização no M2.

Limiar padrão inicial: **0.75** (configurável no M6 via Configurações — ADR-013).

Elementos com similaridade ≥ threshold formam o mesmo cluster. Um elemento pode pertencer a múltiplos clusters (UC-011 FA-002).

---

## Motivações

- **Explicabilidade (ADR-009):** o threshold é completamente transparente — "estes arquivos foram agrupados porque têm 82% de similaridade semântica". DBSCAN produz resultados difíceis de justificar para o usuário sem expor epsilon/minPts.
- **Sem parâmetros ocultos:** o único parâmetro (threshold) tem semântica direta e pode ser exibido na UI futura.
- **Adequado ao M2:** com volumes locais (~centenas a poucos milhares de arquivos), threshold simples funciona bem. DBSCAN seria mais relevante com dezenas de milhares de pontos e padrões de densidade complexos.
- **Alinhamento com UC-011 RN-001/RN-002:** todo cluster tem confiança atribuída — a própria similaridade média dos membros serve como confiança, sem heurísticas adicionais.

---

## Algoritmo

```
Para cada par (A, B) com sim(A, B) >= 0.75:
    Se A e B não pertencem ao mesmo cluster:
        Criar ou expandir cluster

Confiança do cluster = média das similaridades dos pares de membros
```

Implementação: union-find ou grafo de adjacência sobre a matriz de similaridade (ADR-019).

---

## Consequências

### Positivas

- Resultado completamente explicável por similaridade percentual.
- Trivialmente testável (threshold é parâmetro, não heurística).
- Alinhado com a filosofia de conhecimento explicável do projeto.

### Negativas

- Pode gerar clusters muito grandes se o threshold for baixo demais.
- Não detecta automaticamente "outliers" como o DBSCAN faz.

---

## Limiar de Revisão

Se clusters produzidos forem consistentemente inúteis (muito grandes ou muito fragmentados), revisar threshold padrão em M6 com base em dados reais do usuário.

---

## Alternativa Rejeitada

**DBSCAN:** parâmetros epsilon e minPts são difíceis de calibrar sem dados reais, e de explicar ao usuário. Fica como alternativa futura caso o threshold simples mostre limitações em M6.
