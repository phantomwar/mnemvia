# PRD — Mnemvia

Versão 2.2 • 18/09/2026 • Mnemvia • Open source, Apache-2.0 • Estágio: implementação determinística M0.

Este é o PRD vigente do Mnemvia. A [definição de escopo](06-foco-original.md) registra seus limites. Propostas anteriores foram preservadas no [histórico](archive/infrastructure-focus/docs/02-prd.md) e não definem requisitos atuais.

## 1. Produto e propósito

Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

Transforma documentação humana e dados estruturados em conhecimento canônico reutilizável, recupera evidências e prepara contexto para uma IA consumidora. O primeiro público são pessoas com acervos Markdown e desenvolvedores com documentação distribuída entre projetos.

O produto é a camada entre fontes e modelos: **ingestão incremental → bibliotecário/Knowledge Compiler → KIR → índices → recuperação → Context Compiler**. Um chatbot, assistente de programação ou diagnosticador de infraestrutura pode consumir sua saída. Nenhum desses consumidores define sozinho o produto.

A ideia central não é apenas compactar arquivos ou reduzir o número de caracteres. É evitar armazenamento e processamento redundantes, preservar diferenças importantes e entregar a informação necessária com menos tokens, memória e tempo, sem perda injustificada de qualidade.

## 2. Origem e hierarquia de requisitos

O objetivo de produto é otimizar RAG local. A proposta de bibliotecário e elementos comuns entre projetos estabelece ingestão de mudanças, tradução humano–máquina/máquina–humano, representação intermediária e reutilização de conhecimento.

Este PRD define o contrato de produto; o plano organiza sua entrega; o registro de dependências define o estado de adoção. Pesquisa e histórico fornecem evidências e hipóteses, sem substituir requisitos. Exemplos de servidores ilustram um corpus e não especializam o produto em manutenção.

Open source e Apache-2.0 permanecem. CPU e eficiência local são prioridades; o runtime/modelo continuam substituíveis. O perfil M0 `deterministic-lexical` já implementa CLI, SQLite/FTS5, ingestão incremental de Markdown/texto e `composer.json`, revisão por hash, busca lexical, evidência e ContextPackage JSON. Embeddings, vetores, bibliotecário, tokenizer real, API e benchmarks ainda não estão implementados.

O schema v4 também implementa remoção de fontes ausentes, supressão persistente por raiz/escopo/caminho e journal de `ingest`, `suppress` e `unsuppress`, sem apagar o arquivo original. O comando `verify` expõe inconsistências entre fontes, FTS, fatos, supressões e operações pendentes sem alterar o banco. O journal torna falhas e operações pendentes observáveis; replay automático após queda, exclusão em lote e garantia de restore com revogações atuais ainda são requisitos pendentes.

## 3. Usuários e jornadas

| Usuário | Trabalho a realizar | Resultado do produto |
|---|---|---|
| Pessoa com acervo Markdown/Obsidian | Consultar e manter conhecimento sem reindexar tudo | Busca, relações, respostas opcionais e fontes |
| Desenvolvedor com vários repositórios | Reutilizar descrições e dependências compartilhadas | Entidades canônicas, usos, variantes e overrides |
| Desenvolvedor de aplicações de IA | Obter contexto limitado para seu próprio modelo | ContextPackage por API/exportação |
| Pesquisador/contribuidor | Comparar custo e qualidade de estratégias locais | Corpus, runner e resultados reproduzíveis |
| Curador de conhecimento | Corrigir identidade, conflito ou extração | Revisão rastreável e atualização dos derivados |

J1: apontar uma pasta local, ingerir documentos, consultar com um orçamento de tokens e inspecionar as fontes do contexto.

J2: editar um documento e recompilar somente unidades alteradas e seus derivados afetados. A próxima consulta usa uma geração consistente.

J3: importar dois projetos com um componente comum; compartilhar sua definição sem confundir suas instâncias. Alterar um projeto cria/atualiza uma variante ou uso local, preservando o outro.

J4: receber resultados estruturados de uma IA consumidora e propor uma nota Markdown de volta ao acervo. O usuário revisa o diff; uma inferência não se torna verdade só porque foi exportada em prosa.

J5: consumir recuperação/contexto sem executar a LLM de resposta. O motor deve ser útil como biblioteca/serviço de contexto, não exigir uma experiência de chat.

## 4. O que significa “mais otimizado possível”

Não há um ótimo universal. O objetivo é buscar configurações na fronteira de eficiência para corpus, hardware, qualidade mínima e frequência de atualização explicitados.

Medir separadamente:

- **Armazenamento:** bytes das fontes, objetos normalizados, índices, embeddings, resumos, cache e histórico.
- **Ingestão:** tempo inicial, tempo incremental, CPU, memória de pico e chamadas/tokens do bibliotecário.
- **Recuperação:** recall, ranking, latência cold/warm, custo de reranking e expansão de relações.
- **Contexto:** tokens reais no tokenizer consumidor, redundância, preservação de fatos, negações, exceções, números, versões e fontes.
- **Resposta opcional:** acerto, suporte das afirmações, abstenção e tempo completo até a resposta.
- **Custo acumulado:** ingestão + atualizações + consultas + manutenção dos índices.

Comprimir 50% dos tokens e duplicar o tempo total pode ser uma regressão. Deduplicar bytes não prova deduplicação semântica; reduzir tokens não prova melhoria de resposta. Relatar todas as dimensões relevantes, inclusive perdas.

Fórmula de comparação: custo total no período = ingestão inicial + soma das atualizações + soma das consultas + armazenamento/manutenção. Comparar componentes em unidades próprias; não somar segundos, bytes e tokens em um escore sem pesos justificados.

## 5. Escopo inicial

Incluído na alpha: pasta local de Markdown/texto, detecção incremental, parser de um formato estruturado escolhido por fixture, fontes versionadas, KIR mínima extensível, identidade exata, relações, usos/overrides, embeddings locais, recuperação lexical/vetorial, Context Compiler com contagem de tokens e bibliotecário semântico controlado.

O parser estruturado inicial deve demonstrar a pergunta original sobre componentes comuns: manifestos/lockfiles de dois ou mais projetos. O formato específico será fixado no experimento. Conteúdo genérico Markdown permanece suportado; não limitar a ontologia a servidores.

O bibliotecário é um papel lógico: determinismo onde há estrutura; modelo local para prosa, ambiguidades e propostas de resumo. Não exige um segundo modelo sempre residente. Pode compartilhar runtime e operar em fila, com orçamento e prioridade inferior às consultas interativas.

Fora do núcleo: agentes privilegiados, coleta Linux/Windows/Proxmox, reinício de serviços, executor de ferramentas, approvals operacionais, alertas de produção e cálculo de MTTR. São possíveis consumidores/adapters independentes, não marcos obrigatórios.

PDF/OCR, áudio, crawling web, banco distribuído, treinamento, tokenizer próprio e kernels de inferência ficam fora da primeira alpha. Não retirar deduplicação e compilação de contexto do MVP: são a tese a testar.

## 6. Arquitetura

~~~mermaid
flowchart TD
    H[Markdown, Obsidian e dados estruturados] --> D[Detecção de mudanças]
    D --> P[Parse, normalização e classificação]
    P --> K[Knowledge Compiler / bibliotecário]
    L[Modelo local auxiliar opcional] --> K
    K --> IR[KIR canônica com fontes]
    IR --> S[(Source store, relações, FTS e vetor)]
    Q[Consulta, escopo e orçamento] --> R[Recuperação lexical, vetorial e relacional]
    S --> R
    R --> C[Context Compiler]
    C --> CP[ContextPackage com evidências e tokens]
    CP --> A[IA consumidora substituível]
    CP --> U[Inspeção / API / exportação]
    A --> F[Feedback estruturado]
    F --> W[Rascunho Markdown revisável]
    W --> H
~~~

Rust é a direção para o núcleo eficiente. SQLite é o armazenamento inicial candidato. Provedores de embedding, extração, reranking e geração têm responsabilidades separadas, mesmo quando compartilham runtime.

Runtime local externo evita acoplar inferência à primeira implementação. A escolha concreta é fixada por benchmark e disponibilidade, não pelo nome do projeto. Binário nativo e container são formas de distribuição; pesos ficam separados. Não há requisito de vários bancos ou serviços residentes.

Antes de ampliar um motor próprio, comparar a composição de retrieval existente com uma camada Mnemvia de proveniência, variantes e compilação. QMD será a primeira referência local. Rust permanece candidato ao núcleo; escolher a linguagem não constitui evidência de menor custo total. Registrar a decisão construir/compor após o experimento M0.

### 6.1 Dependências e projetos relacionados

O [registro de dependências](../DEPENDENCIES.md) é a referência de classificação. O manifesto Cargo e lockfile do perfil determinístico já existem; nenhum runtime de inferência, modelo, integração externa ou dependência vetorial foi adotado.

| Elemento | Papel | Estado |
|---|---|---|
| Rust | Toolchain/núcleo nativo | Candidato, condicionado à decisão construir/compor |
| SQLite | Persistência local e índices | Candidato; bindings e extensões ainda não escolhidos |
| Runtime e modelo de embeddings | Perfil vetorial local | Implementações/modelos ainda não escolhidos |
| Modelo auxiliar | Bibliotecário semântico | Opcional por perfil, ainda não escolhido |
| Tokenizer consumidor | Orçamento exato de contexto | Capacidade requerida; biblioteca/revisão a selecionar |
| QMD | Baseline local e possível base de composição | Avaliação; não adotado em produção |
| LightRAG | Manutenção de conhecimento compartilhado | Referência e avaliação focada |
| Graphiti, GraphRAG, Cognee | Temporalidade, síntese e memória | Referências; comparação conforme tarefa |
| RAPTOR | Recuperação hierárquica | Referência de pesquisa |
| LLMLingua | Compressão adicional | Candidato a experimento opcional |

Não instalar nem integrar todos esses projetos para entregar o alpha. Benchmark externo não implica dependência de produção. Se QMD for adotado, registrar suas dependências transitivas efetivas; se não for, mantê-lo como comparação externa.

Obsidian é uma origem possível de arquivos Markdown, não aplicativo obrigatório. Consumidores da CLI/API são aplicações independentes; nenhum projeto downstream é confirmado. O caminho lexical não exige modelo de linguagem, enquanto embeddings são necessários ao perfil vetorial. Contexto com orçamento exato requer o tokenizer do consumidor mesmo quando não há geração.

Antes de adotar um componente: fixar revisão, conferir licença/notices, plataformas, comportamento offline, custo, interfaces e estratégia de atualização/substituição. Manifestos e lockfiles serão a autoridade do conjunto instalado quando existirem; o registro narrativo deve acompanhá-los. Não há SBOM de um build inexistente.

## 7. Modelo de dados

| Objeto | Função e invariantes |
|---|---|
| SourceRevision | Fonte, hash, revisão, escopo e intervalos de texto; preserva acesso à evidência |
| ContentObject | Conteúdo normalizado endereçado por hash; igualdade de conteúdo não implica identidade do objeto real |
| Entity / EntityVersion | Conceito ou objeto canônico com chave e versão explícitas |
| Assertion / EvidenceLink | Afirmação e seus múltiplos suportes; classe SOURCE, DERIVED ou INFERRED |
| Relation | Conexão tipada, origem, validade e regra quando derivada |
| Usage / Override | Vínculo local com o canônico e suas diferenças; não altera a definição compartilhada |
| Variant | Elemento distinto com relação explícita à origem, sem inferir herança inexistente |
| Chunk / Embedding | Unidade recuperável, hash, modelo/revisão, dimensão e metadados do índice |
| Summary | Resumo derivado e possivelmente lossy, ligado aos suportes e à versão do extrator |
| Generation | Manifesto consistente de fontes, objetos, índices e versões de processamento |
| ContextPackage | Seleção final, citações, orçamento, tokenizer, omissões e sinal de insuficiência |
| ReviewDecision | Correção humana, merge/split, rejeição ou aprovação com histórico |
| OperationJournal | Intenção durável, alvos, fase e falha de atualização/exclusão; retomada idempotente |
| Revocation / SourceSuppression | Bloqueio de leitura e política para impedir reingestão indesejada de fonte ainda presente |
| ModelFingerprint | Identifica modelo/revisão, dimensão, pooling, normalização, template e pré-processamento |
| RetrievalPlan | Rota, limites, componentes disponíveis, cobertura observada e motivos de fallback |

Exemplo: dois projetos usam a mesma versão de uma biblioteca. A definição/descrição da versão pode ser compartilhada; cada projeto possui Usage com origem no próprio manifesto e lockfile. Uma configuração local diferente é Override; uma implementação efetivamente distinta pode ser Variant. Dois bancos com a mesma versão não são a mesma instância.

Fontes e suportes continuam múltiplos mesmo se o corpo da informação for deduplicado. Excluir uma fonte remove seu suporte, não o conhecimento ainda sustentado por outra. Quando o último suporte desaparece, invalidar derivados e caches correspondentes.

Identidade inclui namespace e escopo. Um hash de configuração idêntica não permite unir produção e homologação. Similaridade vetorial gera candidatos, não prova de equivalência.

Assertions e Relations distinguem observed_at (quando o sistema observou) de valid_from/valid_to (validade sustentada pela evidência). Datas desconhecidas permanecem desconhecidas; mtime não estabelece validade. Uma fonte antiga importada depois não substitui automaticamente um fato recente. Conflitos sem precedência fundamentada permanecem explícitos. No alpha, suportar validade explícita e consulta as_of; inferência temporal complexa fica fora do aceite mínimo.

## 8. Requisitos funcionais

| ID | Requisito | Critério de aceite |
|---|---|---|
| RAG-01 | Ingerir fontes locais autorizadas | Arquivos tratados como dados; sem executar scripts, seguir links remotos ou escapar da raiz por symlink |
| RAG-02 | Detectar add/edit/rename/delete incremental | Reingestão sem alteração não gera novos embeddings/extratos; mudança recompila somente dependentes afetados |
| RAG-03 | Gerar KIR com proveniência | Todo fato publicado liga a fonte ou regra/evidências; formato e extrator versionados |
| RAG-04 | Canonicalizar conteúdo e entidades | Fixtures de igualdade/diferença passam sem confundir entidades por nome/hash de conteúdo |
| RAG-05 | Representar usos, variantes e overrides | Alteração local não modifica consumidores que continuam usando o elemento comum |
| RAG-06 | Implementar bibliotecário semântico local | Saída validada por schema; ambiguidades e extrações sem suporte vão para revisão; orçamento respeitado |
| RAG-07 | Combinar lexical, vetor e relações | Cada modo pode ser isolado no benchmark; consulta exata não depende de busca semântica |
| RAG-08 | Compilar contexto para orçamento/tokenizer | Incluir custos de envelope, referências e definições; sinalizar insuficiência em vez de omitir restrição crítica |
| RAG-09 | Inspecionar e expandir evidências | Permitir chegar ao trecho original, expandir detalhes e explicar a seleção sem depender de raciocínio oculto |
| RAG-10 | Expor contexto sem geração de resposta | Retorno estruturado estável para consumidores locais; referência opaca tem conteúdo resolvível quando autorizado |
| RAG-11 | Rebuild, migração e rollback de geração | Publicação atômica; índice corrompido reconstruível; falha mantém última geração íntegra |
| RAG-12 | Exportar conhecimento para humano | Gerar rascunho/diff Markdown revisável; não sobrescrever silenciosamente fontes ou promover inferência a fato |
| RAG-13 | Isolar acervos e revogar dados | Filtro de escopo antes de retrieval e antes da saída; exclusão invalida respostas/cache/índices afetados |
| RAG-14 | Avaliar custo e qualidade reproduzivelmente | Mesmos corpus, modelos, hardware e budgets; resultados por categoria e logs sanitizados |
| RAG-15 | Operar offline após provisionamento | Ingestão, embedding, recuperação e compilação sem serviço externo obrigatório |
| RAG-16 | Disponibilizar modo sem LLM auxiliar | Parse/retrieval determinísticos funcionam; capacidades semânticas ausentes são declaradas |
| RAG-17 | Reutilizar processamento | Cache por conteúdo, modelo, tokenizer, extrator, escopo e versão; mudança invalida somente entradas dependentes |
| RAG-18 | Incorporar feedback sem contaminar fontes | Feedback/saída do consumidor entra com origem própria e revisão; evitar ciclos de reingestão |
| RAG-19 | Manter lifecycle durável e exclusão segura em lote | Crash em cada fase permite retomada idempotente; revogação bloqueia dados antes da limpeza; cancelamento preserva suportes sobreviventes; estado parcial não aparece como sucesso |
| RAG-20 | Separar tempo de observação e validade | Ingestão fora de ordem não causa regressão silenciosa; consulta histórica preserva conflitos e aplica autorização atual |
| RAG-21 | Verificar compatibilidade de modelos e índices | Mesmo número de dimensões não basta; troca de fingerprint cria geração nova sem misturar vetores; migração incompleta usa fallback declarado |
| RAG-22 | Planejar recuperação por tarefa e recursos | Rotas lexical/híbrida/relacional configuráveis, limites de expansão e registro do plano; consulta global parcial declara lacunas |
| RAG-23 | Degradar de forma controlada quando modelo falha | Timeout, schema inválido, falta de modelo ou memória têm estado explícito e tentativas limitadas; nenhum fallback remoto silencioso |
| RAG-24 | Avaliar português, inglês e corpus misto | PT-BR com acentos, negações, unidades e identificadores; resultados separados por idioma e tipo de pergunta |
| RAG-25 | Validar pré-condições da avaliação | Corpus/índice/tokenizer/modelos incompatíveis invalidam a execução; ausência de resultado numa execução válida continua sendo erro de retrieval |
| RAG-26 | Validar a cadeia de transformação do contexto | Ablação extrativo/estruturado/comprimido no mesmo retrieval; regressões críticas bloqueiam perfil; fallback extrativo ou insuficiência sem exceder orçamento |

### Bibliotecário e precisão semântica

A extração não deve afirmar algo que o texto não diz. SOURCE designa um enunciado explicitamente sustentado, não a certeza de que um modelo o extraiu corretamente. Guardar método de extração e estado de revisão separadamente. Inferência não vira SOURCE por ter sido escrita num resumo.

Hash e chave exata precedem alias/estrutura; modelos entram para ambiguidades restantes. Merges semânticos podem ser propostos e revertidos. O núcleo não força revisão humana de cada registro determinístico.

### Compilação de contexto

Selecionar por relevância, cobertura e orçamento. Preservar negação, quantificadores, exceções, tempo, escopo, dependências e contradições pertinentes. Uma resposta que remove “não” para economizar tokens falha mesmo que a similaridade semântica média pareça alta.

Representação interna compacta não é necessariamente representação adequada ao modelo. KIR Machine pode usar estruturas/binário/banco; KIR View deve ser medida com o tokenizer do consumidor. IDs sem definição economizam texto, mas não transmitem conhecimento sozinhos.

Saída proposta: generation_id, query, scope, tokenizer_revision, budget, used_tokens, evidence_items, selected_facts, source_refs, unresolved_conflicts, omissions e insufficient_context. Metadados volumosos podem ficar fora do prompt, mas referências e definições necessárias ao raciocínio entram na contagem.

Se não couber a evidência mínima, devolver insuficiência e oferecer expansão. Não prometer que toda pergunta admite um contexto minúsculo.

Acrescentar ao ContextPackage retrieval_plan, model_fingerprints, coverage, degraded_reason e as_of quando solicitado. Distinguir execução completa, degradada, contexto insuficiente e falha. Cobertura informa o que foi pesquisado, não uma garantia de que todas as afirmações do corpus foram encontradas. Metadados de inspeção ficam separados da visão enviada ao consumidor, mas tudo que entrar no prompt conta no orçamento.

Preservação de palavras ou dígitos isolados não garante a relação entre sujeito, condição e consequência. Testar spans completos e manter referências até a evidência original, inclusive após múltiplos resumos. Validação automática é parcial; complementar com revisão amostral. Conteúdo de fontes não pode alterar o plano de execução nem permissões.

## 9. Otimizações e ordem de investigação

1. Reaproveitar conteúdo e processamento idênticos.
2. Compilar mudanças por dependência, sem reprocessar todo o corpus.
3. Resolver consultas estruturadas sem LLM quando possível.
4. Filtrar e recuperar melhor antes de comprimir texto.
5. Deduplicar evidências e selecionar granularidade adequada à consulta.
6. Medir reranker/resumidor/compressor incluindo seu próprio custo.
7. Reduzir cópias, alocações, I/O e contenção observados no profiler.
8. Avaliar quantização de embeddings, ANN, mmap e SIMD só nos gargalos demonstrados.

Busca vetorial exata em corpus pequeno é um baseline útil; índice aproximado não é obrigação. Se ANN perder recall relevante, documentar o trade-off. Cache semântico não deve reutilizar resposta de consulta apenas parecida quando números, versões ou permissões divergem.

Armazenamento globalmente compartilhado não pode revelar dados entre namespaces. Na primeira versão, limitar compartilhamento físico ao domínio de confiança, com ACL nas referências.

O plano de recuperação começa por regras explícitas e modos configuráveis. Perguntas exatas podem usar caminho lexical/estruturado; expansão de grafo, reranking e compressão têm budgets próprios e só participam quando habilitados pelo perfil. Um planejador aprendido não é requisito do alpha. Limitar número de candidatos, profundidade e tempo; não confundir coocorrência com relação factual. Síntese hierárquica global é experimento posterior, sem bloquear o caminho inicial.

## 10. Benchmark central do produto

Comparar, no mesmo corpus e com o mesmo modelo consumidor:

| Variante | O que isola |
|---|---|
| B0 — chunks + embeddings + top-k | Baseline RAG simples, ajustado de forma justa |
| B1 — lexical + vetor + dedup | Ganho de recuperação híbrida sem compilação semântica |
| B2 — KIR + relações + Context Compiler | Hipótese central do projeto |
| B3 — B2 com bibliotecário/modelo compressor | Ganho líquido frente ao custo adicional |
| B4 — referência externa selecionada | QMD para retrieval local; LightRAG para manutenção; demais referências conforme tarefa, sem impor sete integrações ao alpha |

Não comparar um B0 mal configurado contra B2 cuidadosamente afinado. Usar conjunto de desenvolvimento separado do teste final; separar avaliação da recuperação de avaliação da geração.

Corpus de fixtures: documentação pessoal genérica, projetos com conteúdo compartilhado, versões divergentes, homônimos, informação negativa, procedimentos com exceções, conteúdo duplicado e atualizações/exclusões. Incluir um conjunto não relacionado a infraestrutura para impedir nova especialização indevida.

Categorias de consulta: identificação exata, busca semântica, relacionamento multi-hop, comparação entre versões, resumo global, procedimento, contradição, pergunta sem resposta. Relatar desempenho por categoria, não apenas média.

Metas propostas para investigar, não resultados: reduzir tokens de contexto em ≥30% frente ao melhor baseline simples, com perda de qualidade não superior a 2 pontos percentuais e sem erro crítico em números/negações/escopo. Avaliar em amostra suficiente e reportar intervalos; vinte perguntas não validam margem de 2 pontos. Na amostra pequena, usar revisão caso a caso e não alegar significância.

Invariantes fortes: zero merge incorreto nas fixtures críticas; fonte rastreável para 100% dos itens selecionados; zero processamento de embedding/extrator no no-op; nenhuma fonte revogada reaparece via cache; alterações comuns invalidam todos e somente os dependentes previstos pelas fixtures.

Pré-voo obrigatório: conferir corpus/coleção, documentos esperados, geração pronta, fingerprints, tokenizer e configurações. Erro de setup produz execução inválida, distinta de qualidade zero. Registrar consultas excluídas e motivos; não melhorar médias removendo silenciosamente os casos difíceis.

Separar PT-BR, inglês e consultas entre idiomas. Acrescentar importação fora de ordem, troca de modelo de mesma dimensão, falha durante exclusão e alteração mantendo nome de arquivo. Medir ablações com o mesmo conjunto recuperado para isolar o efeito da compressão. Resultados divulgados em papers são referências, não metas já demonstradas aqui.

Avaliar taxas de duplicação 0/50/90% e alteração 1/10% em fixtures controladas. Registrar o que mudou além dessas variáveis. Custo amortizado inclui resumos reconstruídos, tentativas falhas, migrações e provisionamento dos modelos; custo recorrente offline é relatado separadamente do download inicial.

## 11. Recursos e perfis de execução

Definir orçamento para Core, índices, embedding, bibliotecário, reranker e consumidor separadamente, e medir o total. Não reservar toda a RAM ao modelo gerador. CPU-only é perfil de referência; 16/32 GB são cenários a testar, não garantias de suporte ou latência.

Testar corpus de 100, 1.000 e 10.000 documentos com tamanhos e duplicação conhecidos; aumentar somente após medir o anterior. Cenário sintético de escala não equivale a qualidade em acervo real.

Registrar p50/p95 cold/warm, RSS e pico agregado, CPU, bytes lidos/escritos, tamanho de índices, tempo de primeira compilação e atualização de 1% do corpus. Embeddings em lotes limitados; backpressure impede jobs do bibliotecário de esgotar memória ou bloquear consultas indefinidamente.

Manifesto explícito define modelos/versões e arquivos necessários. A primeira consulta não baixa pesos silenciosamente. Por tarefa, configurar timeout, limite de tokens e no máximo duas tentativas de extração por padrão; ajustes ficam registrados. Falha de schema ou evidência deixa o extrato pendente/rejeitado, sem impedir uso dos trechos determinísticos seguros. Teste de conformidade inclui validade sintática e suporte factual, pois JSON válido não prova extração correta.

Não fixar TTFT de 3 s nem resposta em 30 s sem hardware definido. Relatório deve distinguir retrieval, compilação, prefill e geração. Otimização é aceita por melhoria no perfil medido com qualidade preservada, não por slogan de “mais rápido”.

## 12. Segurança relevante ao motor

Proteção de fontes, isolamento, classificação, exclusão, cache e prompt injection permanecem essenciais. Documentos e metadados nunca autorizam execução nem alteram política. Evitar segredos na indexação; redigir/quarentenar quando necessário e impedir propagação a logs, exportações e embeddings.

Acesso local não justifica servidor aberto em rede. Uma API futura escuta localmente por padrão; exposição remota requer autenticação e escopo explícitos. Uma CLI privada de usuário único não precisa começar com PKI de frota.

Não enviar dados a provedores externos sem habilitação consciente. Pesos e dependências têm suas próprias licenças. A arquitetura de execução de tools administrativas sai do PRD do núcleo.

### Exclusão, revogação e recuperação

O journal reside no armazenamento autoritativo e acompanha backup/migração; não depende somente da memória do processo ou de cache LLM. Conservar metadados mínimos para recuperação sem duplicar texto sensível desnecessariamente. A limpeza não pode exigir modelo disponível: ocultar/descartar derivados e reconstruir posteriormente dos suportes ainda autorizados.

Após confirmar revogação, checar sua versão em toda leitura/expansão de evidências, inclusive sobre uma geração antiga. Rollback e restore nunca podem reativar acesso revogado. Restaurar backup exige reconciliar o registro de revogações/supressões mais recente antes de servir consultas. Se esse registro não estiver disponível, o restore fica indisponível até revisão explícita; não declarar restauração segura automaticamente.

Exclusão em lote considera apenas operações efetivadas. Reconstruir cada derivado afetado uma vez por conjunto consolidado quando possível, sem apagar suporte sobrevivente por causa de cancelamento. Serializar operações conflitantes sobre a mesma identidade/revisão; retomadas repetidas convergem. Publicar estados como pending, rebuilding, deleting, failed e ready, com erro recuperável visível.

A API distingue retirar do índice, suprimir reingestão e apagar um arquivo fonte. O motor não apaga arquivos originais implicitamente. Limpeza lógica não promete apagamento forense de SSD ou backups; declarar retenção e responsabilidade pelas cópias externas. Histórico temporal não serve como rota de acesso a material revogado.

## 13. Critérios de release

M0: corpus e baseline reproduzíveis, objetivo e perfis definidos.
M1: ingestão incremental, KIR, dedup exata, relações, fontes, delete/rebuild corretos.
M2: retrieval híbrido, Context Compiler, bibliotecário semântico e comparação de qualidade/custo completos.
M3: API/CLI documentadas, retorno Markdown revisável, empacotamento e reprodução por usuário externo.

M0 inclui pré-voo e decisão construir/compor; M1 inclui RAG-19/20/21 com injeção de falha; M2 inclui RAG-22/23/24/26 com perfis degradados; M3 verifica restore após revogação. RAG-25 se aplica a todo relatório. Requisitos adicionais aprofundam correção do núcleo; não exigem integrar todos os concorrentes nem construir um planejador inteligente no alpha.

Sem evidência de ganho, simplificar a técnica que falhou e publicar o resultado; não retirar do objetivo a eficiência de conhecimento/contexto. O produto não precisa conter agentes operacionais para atingir 1.0.

## 14. Decisões ainda abertas

Formato estruturado inicial, modelo local de embeddings, modelo auxiliar, vetor exato/ANN, schema público, biblioteca de tokenização e hardware de referência serão fixados no experimento. A direção Rust/local não autoriza reescrever runtimes de inferência.

Mnemvia é o nome vigente. A triagem inicial e seus limites estão no plano open source. Apache-2.0 e a organização open source permanecem válidas. Nenhuma decisão de dependência deve ser anunciada como concluída antes do registro e da verificação correspondentes.

## 15. Evidências e decisões da revisão comparativa

A [pesquisa comparativa](07-pesquisa-comparativa-e-prevencao.md) registra sete referências, fontes primárias, distinção entre limitações documentadas e hipóteses, correlações e experimentos E01–E08. Ela fundamenta os requisitos RAG-19–26 e os refinamentos acima. Não houve benchmark executado nem reprodução das issues nesta revisão.

Critério de decisão: incorporar uma técnica somente quando resolver um requisito ou demonstrar ganho no perfil escolhido. Se composição com componentes existentes satisfizer os contratos, preferi-la à reimplementação ampla. A tese do produto continua sendo conhecimento reutilizável e contexto local eficiente com evidências.


