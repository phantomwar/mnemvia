# Plano open source — Mnemvia

18/09/2026. Direção vigente: motor local de conhecimento e compilação de contexto para RAG. [PRD](02-prd.md) e [execução](03-plano-de-execucao.md) definem requisitos e gates.

## Nome e posicionamento

Nome escolhido para o planejamento: **Mnemvia**. O nome combina a ideia de memória (`mneme`) com um caminho de recuperação (`via`), sem prender o projeto a um modelo, banco ou interface. Descrição pública: “Local knowledge, reusable facts, efficient context”.

Substitui ContextWeft, cuja metáfora de tecelagem se aproximava de projetos como ContextWeave e produtos Weft. A pesquisa web pelas expressões exatas “Mnemvia” software RAG e “Mnemvia” GitHub não retornou resultados em 18/09/2026. Isso é apenas triagem inicial: não verifica marcas, domínios, todas as plataformas ou projetos não indexados. O repositório público foi criado em [github.com/phantomwar/mnemvia](https://github.com/phantomwar/mnemvia); marca, domínio e namespaces de pacotes ainda não foram reservados. A decisão e o plano de migração estão em [ADR 0002](adr/0002-project-renaming.md).

O diferencial a testar é o ciclo completo: fontes humanas → unidades reutilizáveis com variantes → atualização incremental → contexto econômico com evidências → retorno revisável às fontes. Evitar promessas como “o RAG mais rápido” sem benchmark reproduzível.

## Licença escolhida

**Apache-2.0**, mantida para adoção e integração inclusive comerciais, com concessão expressa de patentes nos termos da licença. Permite derivados proprietários; não exige publicação de mudanças apenas por oferecer um serviço hospedado. É uma escolha deliberada de alcance, não uma proteção contra toda apropriação comercial. [Texto oficial](https://www.apache.org/licenses/LICENSE-2.0.html).

A [política de licenciamento](LICENSING.md) compara MIT, MPL e AGPL e delimita materiais cobertos. Pesos, datasets, documentos privados e dependências não recebem automaticamente a licença do projeto. Não incluir restrições de uso não comercial ou anti-cloud em uma distribuição anunciada como Apache-2.0.

## Núcleo público e limites

Ingestão, KIR, deduplicação, relações, retrieval, compilador, bibliotecário substituível, CLI/API, avaliações, exportação e controles necessários para proteger fontes ficam no projeto público. A utilidade básica não depende de serviço fechado ou chave paga.

Uma futura oferta comercial pode vender hospedagem, suporte ou integrações sem ocultar restauração, isolamento e portabilidade necessários à edição local. Não há serviço comercial lançado ou compromisso de suporte hoje.

## Organização técnica inicial

Um repositório para núcleo, adaptadores locais, CLI/API, schemas, documentação, exemplos e harness de avaliação. Separação lógica antes de criar múltiplos pacotes ou serviços. Parsers, tokenizers, embeddings e runtimes devem ter interfaces substituíveis sem tornar tudo um sistema de plugins no alpha.

Manter um caminho mínimo offline e determinístico. Distribuir pesos separadamente, documentando procedência, licença, tamanho e requisitos. Modelos opcionais não podem fazer download silencioso.

## Comunidade e contribuição

Usar inglês para interfaces públicas e documentação de entrada; aceitar discussões em português. Manter o planejamento detalhado em português e traduzir contratos estáveis quando implementados.

Boas primeiras contribuições:
- Fixtures de documentos sintéticos com fontes e respostas esperadas.
- Casos de duplicatas, variantes, homônimos, negações e exclusão.
- Parsers pequenos com escopo e licença claros.
- Adaptadores de tokenizer e relatórios de benchmark reproduzidos.
- Documentação de instalação verificada em hardware acessível.

Usar DCO 1.1 sem cessão de copyright ou CLA inicial. Um responsável humano deve revisar contribuições assistidas por IA. A política não autoriza republicar documentos de terceiros nem conteúdo da conversa de origem.

## Governança e segurança

Modelo inicial conduzido por mantenedor, com decisões arquiteturais registradas e evolução proporcional à comunidade real. Identificar contas e responsáveis antes da publicação; não simular fundação ou equipe que ainda não existe. [Governança](../GOVERNANCE.md).

Antes do alpha público: estabelecer canal privado de vulnerabilidades, CI com isolamento de segredos, auditoria de dependências/licenças, restauração testada e instruções de reporte. Segurança foca raízes autorizadas, parsers, dados locais, API, caches e isolamento de escopos. [Segurança](../SECURITY.md).

## Lançamento e manutenção

M0 prova o protocolo de comparação; M1 entrega o armazenamento incremental; M2 testa a tese do compilador; M3 entrega o alpha utilizável. Versionar schemas e formatos de exportação desde o início. Reindexação deve ser explícita quando não houver migração segura.

Publicar resultados positivos e negativos, configurações e limitações. Separar custo de instalação/indexação, consulta e geração. Medir saúde por instalações bem-sucedidas, avaliações reproduzidas, correções e capacidade de manutenção; estrelas não comprovam utilidade.

Financiamento possível: patrocínio, bolsas, suporte e trabalho de integração. Não pressupor receita nem disponibilidade regular de voluntários para sustentar prazos.

## Checklist de publicação

- Nome e namespaces verificados/reservados pelo responsável.
- Maintainers e canal privado de segurança reais.
- LICENSE, DCO e notices aplicáveis revisados.
- Demo e corpus redistribuíveis, sem documentos privados.
- Instalação limpa, offline após provisionamento e restore demonstrados.
- Baseline e avaliação reproduzíveis, com limites das conclusões.
- API/schema e compatibilidade documentados.

Nenhum desses itens é apresentado como concluído apenas por constar neste plano. O estado atual é documentação de projeto.


## Descrição e dependências para publicação

Descrição pública em português: Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

Descrição pública em inglês: Mnemvia is an open-source local RAG engine designed to reuse knowledge across documents and compile traceable context within a token budget.

O [registro de dependências](../DEPENDENCIES.md) é a referência de adoção. Rust e SQLite são candidatos; QMD é baseline e candidato de composição; LightRAG e as demais referências não são dependências obrigatórias. LLMLingua é candidato de experimento opcional. Não há integração implementada nem projeto downstream confirmado.

Antes de publicar, conferir a equivalência dos READMEs, o estado dos componentes e os links. Não promover a lista de pesquisa a uma lista de instalação. O histórico permanece identificado como superado.

