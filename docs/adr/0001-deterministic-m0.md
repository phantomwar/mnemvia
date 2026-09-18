# ADR 0001 — Perfil determinístico para M0

Data: 18/09/2026

## Contexto

O Mnemvia precisa de uma base reproduzível antes de avaliar embeddings, modelos auxiliares, grafo ou compressão. O PRD exige uma API de contexto sem geração obrigatória, proveniência e atualização incremental.

## Decisão

Implementar inicialmente um CLI Rust com SQLite embutido e FTS5. O perfil `deterministic-lexical` ingere Markdown, texto e `composer.json`, preserva revisões por hash, extrai requisitos Composer determinísticos, remove fontes que saíram da raiz e retorna um ContextPackage JSON. A varredura não segue links simbólicos.

O schema v4 acrescenta supressão persistente por raiz, escopo e caminho relativo e migra o `operation_journal` para aceitar `ingest`. Suprimir retira a fonte de FTS e fatos canônicos sem apagar o arquivo original; remover a supressão exige reingestão explícita. O journal registra início, conclusão ou falha de `ingest`, `suppress` e `unsuppress`; ingestão usa caminho nulo para representar a raiz inteira. Isso torna operações incompletas observáveis, mas não substitui replay automático após queda ou exclusão em lote prevista no PRD.

O orçamento atual usa `heuristic-v1`, baseado em palavras. É uma marca explícita de limitação, não uma contagem no tokenizer do consumidor; ainda assim, um orçamento finito é aplicado estritamente, contando o marcador de truncamento. Embeddings, vetor, reranking, bibliotecário e geração ficam inativos.

## Consequências

O M0 pode verificar ingestão, no-op, exclusão, busca lexical e evidências localmente. Não pode afirmar qualidade semântica, economia real de tokens ou desempenho vetorial. A seleção de tokenizer e embeddings exige uma ADR posterior, fingerprint de modelo e benchmark por idioma.

QMD continua baseline de avaliação e candidato de composição; não é dependência desta implementação. O manifesto Cargo e [Cargo.lock](../../Cargo.lock) são a autoridade das dependências efetivamente adotadas.
