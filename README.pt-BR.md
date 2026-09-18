# Mnemvia

**Conhecimento reutilizável. Contexto rastreável. Execução local.**

[![CI](https://github.com/phantomwar/mnemvia/actions/workflows/ci.yml/badge.svg)](https://github.com/phantomwar/mnemvia/actions/workflows/ci.yml)

[English](README.md) · [GitHub](https://github.com/phantomwar/mnemvia) · [Dependências](DEPENDENCIES.md) · [Roadmap](ROADMAP.md) · [Contribuição](CONTRIBUTING.md)

Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

O projeto atende acervos Markdown e documentação de projetos: ingere mudanças incrementalmente, preserva conceitos comuns e diferenças locais, recupera evidências e entrega contexto para uma IA consumidora.

**Estado: implementação determinística M0.** O repositório contém um CLI local executável, documentação, pesquisa e políticas. Ainda não há release, retrieval vetorial, bibliotecário semântico, integração de tokenizer do consumidor ou ganho de desempenho medido.

## Funcionalidades planejadas

- Ingerir Markdown/texto local e um formato estruturado de manifesto/lockfile.
- Reutilizar conteúdo e fatos comuns sem fundir entidades ou versões diferentes.
- Preservar revisões, evidências, variantes, incerteza e validade temporal explícita.
- Combinar recuperação lexical, vetorial e relacional com limites de recursos.
- Compilar contexto inspecionável com contagem real de tokens e omissões explícitas.
- Atualizar e excluir conhecimento derivado com recuperação após interrupções.
- Propor alterações Markdown para revisão humana.

Exemplo: dois projetos usam a mesma versão de uma biblioteca, mas têm configurações diferentes. A definição comum pode ser reutilizada; o uso, a configuração e a evidência de cada projeto permanecem separados.

## Fluxo proposto

Documentos locais → ingestão incremental → conhecimento canônico com evidências → recuperação → compilador → pacote de contexto → IA consumidora.

O retorno da IA pode gerar uma proposta Markdown revisável. KIR é a representação intermediária de conhecimento prevista no PRD: organiza afirmações reutilizáveis e seus suportes. Uma extração por modelo continua sendo falível, e as fontes são preservadas.

O bibliotecário semântico é um papel de processamento; não exige dois modelos permanentemente carregados. O perfil lexical determinístico implementado retorna contexto sem gerador de respostas.

## Executar o perfil determinístico M0

Requer Rust 1.94 ou posterior. O primeiro build do Cargo baixa apenas as dependências fixadas do build; ingestão e busca funcionam localmente depois disso, sem baixar modelos nem enviar fontes a um serviço.

~~~sh
cargo run -- init --database .mnemvia/mnemvia.sqlite3
cargo run -- ingest --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo
cargo run -- search --database .mnemvia/mnemvia.sqlite3 --scope demo --query "shared authentication" --budget 80
cargo run -- eval --database .mnemvia/mnemvia.sqlite3 --scope demo --fixture fixtures/m0-corpus/evaluation.json
cargo run -- suppress --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo --path project-atlas/README.md --reason "fonte privada"
cargo run -- unsuppress --database .mnemvia/mnemvia.sqlite3 --root fixtures/m0-corpus --scope demo --path project-atlas/README.md
cargo run -- operations --database .mnemvia/mnemvia.sqlite3
cargo run -- recover --database .mnemvia/mnemvia.sqlite3
cargo run -- status --database .mnemvia/mnemvia.sqlite3
cargo run -- verify --database .mnemvia/mnemvia.sqlite3
~~~

A saída de `search` é um ContextPackage JSON com fontes, orçamento heurístico por palavras e capacidades degradadas explícitas. `heuristic-v1` não é contagem exata no tokenizer, mas orçamentos finitos são respeitados incluindo o marcador de truncamento. Use `inspect --source-id <id>` para abrir a evidência preservada, hashes das revisões, fatos extraídos, estado de supressão e paridade com o FTS. `eval` verifica pré-condições do fixture e informa recall de recuperação; não afirma qualidade semântica de resposta nem desempenho de benchmark.

`suppress` remove uma fonte da recuperação e registra um bloqueio persistente de reingestão para aquela raiz e escopo. Não apaga o arquivo original. `unsuppress` remove apenas o bloqueio; execute `ingest` depois para reativar uma fonte existente.

O schema também registra operações de ingestão e supressão como `started`, `completed` ou `failed`; use `operations` para inspecioná-las. Uma operação de ingestão com `path: null` representa a raiz inteira. Ingestão e supressão confirmam o journal na mesma transação da mutação das fontes; se houver interrupção antes do commit, ambas são revertidas e a operação permanece visível como `started`. Replay automático e recuperação completa de exclusão em lote continuam pendentes.

`verify` é uma verificação de integridade somente leitura para paridade entre fontes e FTS, suporte de fatos, política de supressão e operações pendentes. Ele informa se o banco está consistente; não corrige dados automaticamente.

`recover` marca como falhas as operações deixadas em `started` e informa como repetir o comando. Ele é conservador e idempotente: não presume se uma transação antiga chegou a confirmar, e repeti-lo não altera operações já recuperadas. Revise o banco e execute o comando original explicitamente.

## Dependências e projetos relacionados

O perfil determinístico adotou Rust, SQLite/FTS5 embutido e as crates diretas do [DEPENDENCIES.md](DEPENDENCIES.md). Modelos, bibliotecas de tokenização e runtime de inferência continuam sujeitos à avaliação.

| Projeto | Relação planejada |
|---|---|
| QMD | Baseline de busca local e candidato para composição |
| LightRAG | Referência/comparação para manutenção de conhecimento compartilhado |
| Graphiti, Microsoft GraphRAG e Cognee | Pesquisa e possíveis avaliações específicas |
| RAPTOR | Referência para recuperação hierárquica |
| LLMLingua | Candidato a experimento opcional de compressão |

Essa lista não é um conjunto de pacotes obrigatórios. Não há integração implementada nem projeto consumidor confirmado. O [registro de dependências](DEPENDENCIES.md) detalha links oficiais, classificação, perfis de execução e critérios de adoção.

Uma pasta do Obsidian é uma possível fonte; o aplicativo Obsidian não é dependência do núcleo planejado. O fluxo local não exige nuvem ou API paga.

## Escopo e validação

A primeira entrega terá CLI e API em loopback. Diagnóstico de infraestrutura pode consumir o contexto, mas monitoramento de hosts e execução de comandos ficam fora do núcleo. PDF/OCR, áudio e crawling web não fazem parte da primeira alpha.

A validação compara um baseline simples competitivo, QMD, conhecimento canônico e compressão semântica opcional. Qualidade, tokens, memória, latência, indexação inicial e manutenção incremental serão medidos em conjunto.

Antes de construir todo o mecanismo de busca, o plano verifica se uma base existente com a camada de proveniência e compilação do Mnemvia atende aos requisitos.

## Documentação

- [Guia da documentação](docs/README.md): especificações vigentes e histórico.
- [PRD](docs/02-prd.md): requisitos, contratos e critérios de aceite.
- [Plano de execução](docs/03-plano-de-execucao.md): etapas, dependências e experimentos.
- [Pesquisa comparativa](docs/07-pesquisa-comparativa-e-prevencao.md): fontes, correlações e prevenção.
- [Registro de dependências](DEPENDENCIES.md): candidatos e componentes efetivamente adotados.
- [Plano open source](docs/05-plano-open-source.md): distribuição e comunidade.

## Participação e segurança

Contribuições úteis nesta fase incluem corpus sintético, perguntas com evidências esperadas, casos de variantes e propostas de avaliação. Discussões em português ou inglês são bem-vindas.

Consulte [contribuição](CONTRIBUTING.md), [governança](GOVERNANCE.md), [código de conduta](CODE_OF_CONDUCT.md) e [segurança](SECURITY.md). Não envie documentos privados, segredos ou datasets sem direito de redistribuição. O canal privado de segurança ainda precisa ser estabelecido antes do alpha público.

## Licença

Materiais originais usam [Apache-2.0](LICENSE), salvo indicação específica. Componentes externos, dados de entrada e pesos mantêm suas próprias licenças. Veja a [política de licenciamento](docs/LICENSING.md).

Mnemvia é o nome escolhido. O repositório público está em [github.com/phantomwar/mnemvia](https://github.com/phantomwar/mnemvia). Nenhuma marca, domínio ou namespace de pacote foi registrado neste planejamento.

