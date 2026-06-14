# ADR-020 — Serialização de Vetores: BLOB Binário f32 Little-Endian

**Status:** Aceita  
**Data:** 2026-06-13  
**Marco:** M2 — Conhecimento

---

## Contexto

A tabela `embeddings` do esquema SQL armazena vetores gerados por UC-009. O formato de serialização afeta tamanho em disco, performance de leitura e compatibilidade futura com extensões vetoriais.

Duas opções foram consideradas:
- **BLOB binário f32 LE:** vetor como sequência de bytes brutos (4 bytes por dimensão).
- **JSON array de floats:** vetor como texto `[0.12, -0.34, ...]`.

---

## Decisão

Serializar vetores como **BLOB binário em f32 little-endian**.

Para nomic-embed-text com 768 dimensões: 768 × 4 bytes = **3.072 bytes por vetor**.

---

## Motivações

- **Compacto:** ~5× menor que JSON equivalente (3 KB vs ~15 KB por vetor de 768 dims).
- **Sem overhead de parsing:** leitura direta como slice de `f32` em Rust; sem JSON parsing a cada busca.
- **Padrão do setor:** formato usado por sqlite-vss, pgvector e a maioria das libs vetoriais — facilita migração futura (ADR-019).
- **Determinístico:** sem variações de formatação de float entre plataformas.

---

## Consequências

### Positivas

- Performance de leitura máxima: `bytemuck::cast_slice` transforma BLOB em `&[f32]` sem cópia.
- Tamanho total previsível: 1.000 arquivos indexados = ~3 MB em embeddings.
- Compatível com sqlite-vss se migrado no M6.

### Negativas

- Não é legível diretamente em um editor SQL; requer ferramenta para inspecionar o vetor.
- Sensível a endianness: documentado como little-endian, deve ser explicitado na leitura/escrita.

---

## Implementação

```rust
// Serialização
let bytes: Vec<u8> = bytemuck::cast_slice(&vetor_f32).to_vec();

// Deserialização
let vetor: Vec<f32> = bytemuck::cast_slice::<u8, f32>(&blob).to_vec();
```

Crate necessária: `bytemuck` (zero-copy cast entre tipos de bytes).

---

## Alternativa Rejeitada

**JSON array:** 5× maior em disco, requer parsing a cada operação de similaridade, incompatível com sqlite-vss.
