# Plano de execução — RAG local e compilador de contexto

Versão 2.2 — 18/09/2026 — Mnemvia. Autoridade de requisitos: [PRD](02-prd.md). Limites do produto: [escopo](06-foco-original.md). A [pesquisa comparativa](07-pesquisa-comparativa-e-prevencao.md) fundamenta prevenção de falhas e decisão construir/compor. O [registro de dependências](../DEPENDENCIES.md) separa candidatos, referências e adoção efetiva. O corte inicial implementado é o perfil `deterministic-lexical` do M0.

## 1. Resultado a entregar

Uma pessoa aponta uma pasta local, indexa seus documentos, consulta a base e recebe um pacote de contexto rastreável dentro de um orçamento de tokens. Alterações atualizam apenas dependências afetadas. Conceitos compartilhados são reutilizados sem apagar versões, divergências ou evidências. Uma IA externa pode consumir o pacote, mas não é necessária para testar o mecanismo de recuperação.

Descrição de produto: Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

A primeira entrega é CLI e API local. Interface gráfica, agentes de infraestrutura e execução de comandos não condicionam o lançamento.

## 2. Método e capacidade

Trabalhar por incrementos verificáveis, com uma pessoa responsável por integração e revisão. Estimativa inicial para um desenvolvedor experiente dedicado: 12–20 semanas de esforço, sujeita ao experimento inicial; tempo de colaboradores voluntários não deve ser tratado como capacidade garantida. São faixas de planejamento, não compromissos públicos de data.

Cada etapa termina com demonstração reproduzível, resultado medido e decisão registrada. Um ganho de compressão que prejudica as respostas ou aumenta desproporcionalmente o custo total não passa automaticamente.

As faixas existentes precisam ser reestimadas após M0 considerando os requisitos de recuperação e temporalidade da v2.1; não absorver trabalho adicional fingindo prazo garantido. A primeira temporalidade cobre datas explícitas e conflitos, sem raciocínio temporal geral; o primeiro planejador usa regras configuráveis. Não implementar sete integrações para concluir a pesquisa.

## 3. M0 — Corpus, contrato e baseline (1–2 semanas)

Entregas:
- Corpus sintético redistribuível com documentos gerais, versões e dois projetos que usam componentes comuns, com configurações diferentes.
- Perguntas factuais, relações entre documentos, exceções, negações, mudanças temporais e perguntas sem resposta.
- Relevância esperada por pergunta, evidências mínimas e resposta de referência quando aplicável.
- Separação entre desenvolvimento e avaliação; nenhum ajuste do compressor com o conjunto final.
- Baseline B0 de chunks + embeddings + top-k e B1 com busca lexical/híbrida e deduplicação simples.
- Harness que mede indexação, atualização, disco, memória, latência e tokens usando tokenizer identificado.
- ADRs para stack, formatos iniciais, identidade de entidades, contratos públicos e ambiente de teste.

Aceite: executar o mesmo corpus duas vezes, registrar versões/seeds/configuração e produzir relatório comparável. Congelar um baseline simples competitivo antes de atribuir ganhos ao KIR.

Decisão: Rust e SQLite são candidatos para o núcleo local. Validar bibliotecas, esforço e portabilidade em uma pequena prova; não antecipar otimizações de baixo nível. O protótipo pode usar ferramentas de avaliação separadas do núcleo.

Adições da pesquisa: executar E01 com QMD como baseline externo local, congelando tag/commit, modelos e parâmetros. Verificar corpus/índice/tokenizer antes de medir: configuração inválida interrompe o experimento, não produz recall zero. Preparar fixtures PT-BR/inglês/mistas desde o início. Registrar ADR construir/compor: se retrieval existente mais camada de proveniência/compilação cumprir o perfil, preferir composição a uma reimplementação completa.

Saída obrigatória de M0: matriz de dependências revisada. Para cada componente escolhido, documentar papel de build/runtime/avaliação, revisão, licença, dependências transitivas, plataforma e teste de integração. Criar manifestos/lockfiles apenas para a implementação selecionada. Fixar perfis e modelos/tokenizers; atualizar README e DEPENDENCIES.md junto à adoção do componente.

## 4. M1 — Armazenamento canônico e ingestão incremental (3–5 semanas)

Ordem de trabalho:
1. Implementar identidade de fonte, revisão por hash, escopo autorizado e parser de Markdown/texto.
2. Persistir objetos, entidades, fatos, evidências e relações com schema versionado.
3. Acrescentar um parser de manifesto/lockfile escolhido em M0 para validar componentes compartilhados.
4. Separar identidade semântica de hash de conteúdo; igualdade textual não significa mesma entidade.
5. Implementar Usage/Override e versões: componente comum uma vez, vínculo por projeto e divergência explícita.
6. Construir dependências de derivados e invalidação transacional por geração.
7. Implementar remoção, revogação de escopo, reconstrução e recuperação após interrupção.

Aceites obrigatórios:
- Reimportar sem mudanças não recalcula embeddings nem chama modelo.
- Alterar um arquivo invalida somente os derivados afetados, com contadores auditáveis.
- Excluir uma das duas fontes de um fato preserva o suporte restante; excluir todas remove o fato dos resultados.
- Duas versões incompatíveis não são fundidas. Overrides não alteram o componente comum.
- Uma consulta não mistura gerações parcialmente publicadas.
- Arquivo ignorado, fora da raiz ou em outro namespace não vaza por busca, relações ou cache.

Dependência: M0. Relações em SQLite bastam para o experimento inicial; adotar banco de grafos só após demonstrar necessidade.

### Aceites adicionais M1 — integridade antes de otimização

- RAG-19: journal durável, estados parciais, revogação imediata e retomada. E03 encerra o processo em cada fase; retry e cancelamento de lote não apagam suportes sobreviventes. Cache vazio e modelo desligado não impedem revogar/excluir.
- RAG-20: E04 importa fatos com datas explícitas fora de ordem e verifica consulta atual/histórica; validade desconhecida não é preenchida com data de ingestão.
- RAG-21: E08 troca modelos de mesma dimensão e interrompe a migração; vetores incompatíveis nunca se misturam. Fallback lexical aparece na saída.
- E02 varia duplicação e mede processamento evitado sem merges incorretos; mudança de conteúdo mantendo nome e renomeação são casos separados.
- A mesma fonte não pode ser reingerida durante exclusão conflitante. Supressão persistente impede que uma nova varredura ressuscite documento retirado por política.

Responsável: mantenedor do núcleo. Evidência exigida: fixtures, logs sanitizados de fases, estado final e contadores de recomposição; não basta demonstrar exclusão bem-sucedida sem interrupção.

Estado atual: o perfil M0 implementa remoção da fonte ausente, `suppress`/`unsuppress` persistentes, journal visível para `ingest`, `suppress` e `unsuppress`, conclusão do journal na mesma transação da mutação de fonte, inspeção de revisões/fatos/supressão/FTS, marcação manual e idempotente de operações interrompidas com `recover`, verificação de integridade somente leitura e migração v3→v4. O journal expõe uma operação interrompida e `verify` identifica divergências; replay automático, batches, recuperação completa após queda e reconciliação de backup continuam pendentes. Não tratar essa primeira fronteira como cumprimento integral de RAG-19.

## 5. M2 — Recuperação, bibliotecário e compilador (4–7 semanas)

Entregas:
- Busca lexical e vetorial independentes, fusão configurável e expansão de relações com limites.
- Context Compiler que seleciona evidência por utilidade, remove repetição, preserva restrições e respeita orçamento real.
- Pacote com tokenizer, geração, fatos, trechos de evidência, referências, omissões e conflitos.
- Inspeção/expansão: o consumidor consegue abrir o trecho original de um fato resumido.
- Bibliotecário semântico local opcional para relações ambíguas e resumos; saída validada por schema e ligada às fontes.
- Modos determinístico, semântico e ablações comparáveis.
- Cache por fonte, versão de parser/modelo/prompt, escopo e parâmetros de consulta.

O bibliotecário faz parte da hipótese central, mas não precisa ser invocado em cada arquivo nem permanecer residente. Templates estruturados e extração determinística vêm antes da chamada a modelo. Um identificador curto só reduz tokens se o consumidor recebe definições suficientes para interpretá-lo.

Aceite:
- Orçamento respeitado incluindo cabeçalhos, identificadores e referências; quando insuficiente, resposta estruturada indica a limitação.
- Negação, números, versões, datas e exceções essenciais preservados no conjunto de regressão.
- Conteúdo sem evidência não vira fato confirmado.
- Comparação B1/B2/B3 apresenta qualidade, memória, custo de construção e custo por consulta, inclusive quando a otimização perde.
- Meta exploratória: reduzir pelo menos 30% dos tokens frente ao melhor baseline simples com queda de qualidade de no máximo 2 pontos percentuais; dimensionar amostra e reportar incerteza. Não é promessa de desempenho.

Dependência: M1 para rastreabilidade e invalidação. Retrieval simples pode começar em paralelo à modelagem dos dados, sem exigir trabalho de agentes paralelos.

### Aceites adicionais M2 — custo e perda semântica

RAG-22 implementa rotas configuráveis e limites de candidatos, expansão e tempo; pergunta global não apresenta top-k como cobertura integral. RAG-23 aplica timeout e até duas tentativas de extração por padrão, sem escalada remota; erro de modelo permite caminho determinístico seguro, identificado como degradado.

E05 compara contexto extrativo, estruturado e comprimido no mesmo retrieval; preserva condição, unidade, negação e exceção (RAG-26). Fallback cabe no orçamento ou devolve insuficiência. E06 mede PT-BR, inglês e consulta entre idiomas separadamente (RAG-24). E07 mede falhas de modelo, memória e orçamento. Relatório inclui tempo de tentativas e etapas que não produziram contexto útil.

Responsável: mantenedor de retrieval/avaliação, que pode ser a mesma pessoa do núcleo nesta fase. Não apresentar revisão própria como validação independente. Hierarquias de resumos e comparação completa com GraphRAG/RAPTOR ficam como experimentos condicionais, sem bloquear o alpha.

## 6. M3 — Produto local utilizável (2–4 semanas)

Entregas:
- CLI para ingestão, atualização, consulta, inspeção, exportação e diagnóstico do índice.
- API em loopback com acesso controlado e contratos de contexto independentes do provedor gerador.
- Fluxo de retorno: proposta estruturada → revisão humana → alteração em Markdown → nova revisão indexada.
- Histórico distingue documento fonte, inferência e conteúdo gerado; impedir ciclos em que a saída do modelo se confirma sozinha.
- Configuração portátil, backup/restauração, migração e erro de falta de espaço.
- Instruções de instalação verificadas, demo sintética e perfil de recursos.
- Artefatos de release, inventário de dependências, licenças/notices e canal privado de segurança.

Aceite: instalação em ambiente limpo da plataforma escolhida; ingestão e consulta offline após provisionamento explícito das dependências/modelos; nenhum download oculto. Um consumidor recebe contexto sem exigir geração de resposta dentro do projeto.

Verificação adicional: restore de backup anterior a uma revogação não libera o dado novamente. Reconciliar revogações atuais antes de abrir consultas; sem esse registro, exigir revisão explícita do restore. Documentar o alcance da exclusão lógica e retenção de backups. A API distingue retirar do índice e suprimir reingestão; não apaga arquivos originais implicitamente.

## 7. Backlog priorizado

| Prioridade | Item | Requisito/resultado |
|---|---|---|
| P0 | Corpus, baseline e harness | RAG-14, decisão baseada em qualidade e custo |
| P0 | Fontes, revisões, escopos e evidências | RAG-01/02/03/13 |
| P0 | Identidade, reutilização e variantes | RAG-04/05 |
| P0 | Invalidação, exclusão e restauração | RAG-02/11/13 |
| P0 | Retrieval híbrido e ContextPackage | RAG-07/08/09/10 |
| P0 | Bibliotecário validado e modo determinístico | RAG-06/16 |
| P1 | Cache e instrumentação por etapa | RAG-17 |
| P1 | Revisão humana e retorno ao Markdown | RAG-12/18 |
| P1 | Empacotamento e offline demonstrável | RAG-15 |
| P0 | Journal, revogação e retomada de exclusão | RAG-19; M1/M3; E03 |
| P0 | Validade explícita e ingestão fora de ordem | RAG-20; M1; E04 |
| P0 | Fingerprint e migração vetorial | RAG-21; M1; E08 |
| P0 | Pré-voo e baseline QMD | RAG-25; M0; E01 |
| P0 | Perfis de recuperação e falha de modelo | RAG-22/23; M2; E07 |
| P0 | Corpus por idioma e ablações de transformação | RAG-24/26; M2; E05/E06 |
| P2 | Novos parsers, rerankers e compressão hierárquica | Apenas com benchmark favorável |
| Fora do núcleo | Monitoramento de hosts e execução operacional | Consumidores independentes |

## 8. Estratégia de testes

Testes unitários de identidade, escopo, parser, orçamento e invalidação. Testes de integração da ingestão até o pacote de contexto com dados sintéticos. Testes de falha entre etapas transacionais e de migração/restore. Avaliações de retrieval separadas de avaliação da resposta gerada para não confundir melhora do modelo com melhora da base.

Manter fixtures adversariais: instruções embutidas nos documentos, link simbólico para fora da raiz, entidades homônimas, evidência contraditória, duplicata com restrição diferente, fonte removida e orçamento mínimo. Avaliador LLM pode auxiliar, mas não ser o único juiz de correção.

Não repetir benchmarks completos em toda edição de documentação. Executar verificações rápidas por PR e suites maiores quando ingestão, retrieval, schemas, modelos ou compilação mudarem.

## 9. Relatório de desempenho

Publicar hardware, SO, corpus e licença, versão do código, tokenizer/modelos e configurações. Medir p50/p95 de consulta e atualização, cold/warm, pico de memória agregado dos processos, tamanho do índice, embeddings recalculados, tokens de contexto e qualidade por tipo de pergunta. Informar tempo inicial e número de consultas necessário para amortizar uma compilação cara.

Escalas exploratórias: 100, 1.000 e 10.000 documentos; tamanho e duplicação também precisam constar. Perfis de 16 e 32 GB são ambientes a testar, não requisitos já comprovados. Otimizar apenas o gargalo identificado pelo profiler.

## 10. Riscos e respostas

| Risco | Resposta e decisão |
|---|---|
| Canonicalização une fatos diferentes | Chaves de identidade explícitas, versões e revisão de merges ambíguos |
| Compressão elimina ressalvas | Evidência expansível e conjunto de regressão para restrições |
| Bibliotecário custa mais que economiza | Incrementalidade, cache e comparação de custo amortizado |
| Modelo pequeno erra extração | Validação, abstenção e fallback determinístico |
| Atualização deixa resultados antigos | Gerações atômicas e invalidação de cache/derivados |
| Base fica acoplada ao gerador | API de contexto e adaptadores separados |
| Escopo cresce além da manutenção voluntária | Formatos limitados, gates e integrações fora do núcleo |

## 11. Critério de lançamento e continuidade

Publicar alpha quando M0–M3 tiverem evidência de funcionamento e limitações conhecidas. É aceitável lançar com ganhos modestos demonstrados; não é aceitável anunciar superioridade sem comparação. Antes de 1.0, estabilizar schemas/API, política de compatibilidade e restauração em mais de uma release.

Se KIR ou compressão não superar B1 com qualidade preservada, simplificar a representação ou restringir os casos de uso e repetir o experimento. O objetivo é eficiência local útil, não defender uma arquitetura a qualquer custo.

## 12. Rastreabilidade da pesquisa

O relatório [07](07-pesquisa-comparativa-e-prevencao.md) contém evidências D/R/H, problemas P01–P08 e experimentos E01–E08. Não há resultado numérico executado nesta revisão. O trabalho atual terminou na especificação das prevenções; demonstrá-las é um gate de implementação e release.

Antes de usar código de qualquer referência, verificar licença e manutenção na revisão congelada. Benchmark externo pode usar adaptador de avaliação sem tornar o projeto uma dependência de produção. Issues abertas são ponto de partida para fixtures, não justificativa para anunciar defeitos de todas as versões concorrentes.

## 13. Documentação e entrega ao repositório

O README apresenta descrição, estado real, fluxo e dependências; o PRD define requisitos; DEPENDENCIES.md registra relações técnicas; ROADMAP resume os marcos. A cada release, atualizar instalação, versões suportadas, limitações, matriz de capacidades e licenças a partir do build real.

Antes de publicação, conferir links, nomes, coerência PT-BR/inglês, ausência de dados privados e separação do histórico. Não apresentar integração, parceria, benchmark ou dependente downstream inexistente como realizado. Políticas de governança e reporte devem identificar responsáveis reais quando estabelecidos.

