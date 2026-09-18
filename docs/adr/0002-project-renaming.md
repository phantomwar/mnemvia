# ADR 0002 — Renomear o projeto para Mnemvia

**Data:** 18/09/2026  
**Status:** aceito para o estágio pré-release

## Contexto

O nome anterior, ContextWeft, explicava uma metáfora de contexto, mas não comunicava diretamente um motor RAG local, reutilização de conhecimento ou proveniência. A triagem comparativa também encontrou nomes próximos e já usados no espaço de contexto e RAG, como [ContextWeave](https://github.com/OpenMOSS/ContextWeave), [WeftlineAI](https://weftlineai.com/) e [Weft](https://withweft.com/).

## Decisão

Adotar **Mnemvia** como nome público e `mnemvia` como slug de repositório, crate e executável. O nome é uma composição curta de memória e caminho; o significado apoia a recuperação rastreável sem limitar a arquitetura a embeddings, SQLite, Rust ou um provedor de modelos.

O posicionamento público permanece: “Local knowledge, reusable facts, efficient context”. A descrição operacional permanece “motor RAG local e open source para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens”.

## Consequências

- O crate passa a ser `mnemvia-cli` e o executável `mnemvia`.
- O diretório de estado padrão passa a ser `.mnemvia/mnemvia.sqlite3`.
- A documentação atual, manifestos e exemplos usam Mnemvia; o material em `docs/archive/` continua preservando o histórico anterior.
- Bases antigas continuam utilizáveis quando fornecidas explicitamente com `--database`; não há compatibilidade implícita com o caminho padrão antigo.
- A licença Apache-2.0 não muda.

## Limites da decisão

Esta é uma decisão de produto para o estágio pré-release, não uma liberação de marca. Antes de publicar ou registrar o namespace, repetir a busca em GitHub, crates.io, PyPI, npm, domínios e bases de marcas nos mercados relevantes.
