# Pesquisa comparativa e prevenção de falhas — Mnemvia

Data de consulta: 18/09/2026. Escopo: sete projetos/técnicas relacionados a RAG local, memória estruturada e compilação de contexto. Resultado: revisão documental com propostas de engenharia, não benchmark executado nem auditoria completa de código.

## 1. Conclusão para o projeto

A proposta faz sentido, mas busca local, grafos de conhecimento, memória temporal e compressão já possuem implementações relevantes. O diferencial a validar é combiná-los com **reutilização canônica, variantes explícitas, atualização correta e orçamento de recursos verificável**, preservando a qualidade das evidências.

A primeira comparação externa deve ser QMD para recuperação local; LightRAG para manutenção de conhecimento compartilhado. Graphiti serve para semântica temporal; GraphRAG e RAPTOR para perguntas globais; Cognee para integração da memória; LLMLingua para compressão adicional. Nenhum deles deve ser declarado inferior por não perseguir exatamente o mesmo objetivo.

Mnemvia permanece o nome escolhido. Não houve publicação, reserva de namespace ou comprovação de superioridade.

## 2. Método e limites

Fontes: repositórios e documentação oficiais, código apontado pelo Context7, artigo dos autores e uma RFC no repositório oficial. O Context7 foi consultado para QMD, LightRAG, Graphiti, Cognee, GraphRAG e LLMLingua. RAPTOR foi consultado no artigo original.

Classificação usada:
- **D — documentado:** capacidade ou limitação descrita pelo próprio projeto.
- **R — relato:** problema descrito em issue/RFC; não reproduzido nesta máquina.
- **H — hipótese:** risco ou solução inferida nesta revisão; requer teste.

As páginas de branches principais são mutáveis. Registrar tag/commit, configuração, dependências e modelos no momento do benchmark. A data desta pesquisa não substitui esse congelamento. Não converter números publicados com outros modelos/hardware em estimativa do Mnemvia.

## 3. Mapa de similaridade

| Referência | Relação com o objetivo | Onde comparar | Limite da comparação |
|---|---|---|---|
| QMD | Busca local de documentos, especialmente Markdown | Retrieval e custo local | Não presumir equivalência com KIR e variantes |
| LightRAG | Entidades/relações e manutenção do grafo | Conhecimento compartilhado e exclusão | “Incremental” não garante baixo custo de toda operação |
| Graphiti | Conhecimento temporal e proveniência | Mudanças, conflitos e consultas históricas | Memória episódica não é igual a pasta de documentos |
| Microsoft GraphRAG | Extração e síntese de comunidades | Respostas globais sobre um corpus | Custo/qualidade dependem do método escolhido |
| Cognee | Pipeline modular de memória e permissões | Ingestão, integração e isolamento | Comparar uma configuração concreta, não todas as opções |
| RAPTOR | Recuperação em níveis de resumo | Questões que exigem visão de conjunto | Artigo não demonstra o perfil local desta proposta |
| LLMLingua | Compressão do contexto recuperado | Custo líquido e retenção de informação | Compressor não resolve origem, exclusão ou identidade |

## 4. Evidência por projeto

### QMD

**D:** combina busca lexical, vetorial e reranking local. A documentação exige novos embeddings ao trocar o modelo e alerta que um benchmark sem a coleção indexada pode produzir zeros sem aviso. Também documenta controle de confiança para configurações locais capazes de acessar recursos externos. [README oficial](https://github.com/tobi/qmd).

**H — consequência para Mnemvia:** começar por um baseline local forte; validar pré-condições do benchmark; tratar configuração importada como dado não confiável; identificar integralmente o espaço vetorial. O custo de cada etapa extra deve aparecer no relatório. Isso não demonstra que a busca do QMD seja lenta ou inadequada.

### LightRAG

**D:** a exclusão por documento remove contribuições exclusivas e reconstrói elementos compartilhados. [Documentação do núcleo](https://github.com/HKUDS/LightRAG/blob/main/docs/ProgramingWithCore.md).

**R:** a RFC #3659, aberta na consulta, descreve reconstruções repetidas em exclusão em lote, estados parciais pouco visíveis e problemas de recuperação de propostas anteriores. É um relato/projeto de correção, não uma falha que reproduzimos nem prova de que toda release tem o mesmo comportamento. [RFC e discussão](https://github.com/HKUDS/LightRAG/issues/3659).

**H — consequência:** suporte das afirmações precisa ser dado durável; cache não pode ser a única evidência para reconstrução. Exclusão necessita intenção persistente, retomada idempotente, visibilidade de estado e bloqueio imediato da informação revogada. Consolidar trabalho por objeto compartilhado, respeitando cancelamentos e fontes sobreviventes.

### Graphiti

**D:** o projeto mantém fatos temporais e proveniência; o README alerta que modelos menores podem produzir schemas incorretos e falhas na ingestão. A implantação exige escolher e operar um backend compatível; não se deve confundir o framework aberto com o serviço Zep. [Repositório oficial](https://github.com/getzep/graphiti).

O modelo distingue validade do fato e tempo de invalidação no sistema. [Campos temporais](https://github.com/getzep/graphiti/blob/main/graphiti_core/edges.py).

**H — consequência:** separar quando algo era válido de quando foi descoberto; não invalidar uma afirmação apenas porque outra foi importada depois. Modelos locais precisam de teste de conformidade, tentativas limitadas e caminho determinístico. Um JSON válido ainda pode conter uma extração errada.

### Microsoft GraphRAG

**D:** Standard usa extração semântica; FastGraphRAG substitui parte dela por NLP e coocorrência. A documentação reconhece um grafo mais ruidoso no método rápido e limitações linguísticas das opções padrão. [Métodos oficiais](https://microsoft.github.io/graphrag/index/methods/).

Há pipelines incrementais; portanto, a caracterização “só reindexa tudo” seria incorreta. [Fábrica de workflows](https://github.com/microsoft/graphrag/blob/main/packages/graphrag/graphrag/index/workflows/factory.py).

**H — consequência:** manter modos baratos e semânticos comparáveis; proximidade textual não deve virar relação factual. Testar edição preservando nome, renomeação e exclusão separadamente: existência de update não comprova todas essas semânticas. Não inferir um bug atual apenas por um trecho de código isolado.

### Cognee

**D:** oferece eliminação de memória por item/dataset e configuração de controle de acesso. O exemplo mínimo de implantação desativa autenticação para teste individual, explicitamente distinguindo esse cenário do uso multiusuário. [Interface de memória](https://github.com/topoteretes/cognee/blob/main/cognee-mcp/README.md), [configuração mínima](https://github.com/topoteretes/cognee/blob/main/docs/minimal-docker-compose.md).

**H — consequência:** demonstração simples não pode ser confundida com configuração de serviço compartilhado. Namespace organiza dados; autorização decide quem pode recuperá-los. Todo caminho de retrieval, expansão, inspeção e cache deve aplicar o mesmo limite. Não foi constatado vazamento no Cognee nesta pesquisa.

### RAPTOR

**D:** o trabalho constrói uma hierarquia mediante embeddings, agrupamento e resumos recursivos, permitindo recuperar diferentes níveis de abstração. [Artigo dos autores](https://arxiv.org/abs/2401.18059).

**H — consequência:** resumos podem ajudar perguntas globais, mas devem continuar ligados às folhas de evidência. Uma alteração precisa invalidar os ancestrais afetados. Avaliar contaminação por resumo antigo e preservar detalhes críticos fora do resumo quando necessário. Estes são riscos de aplicação da técnica, não bugs demonstrados do projeto.

### LLMLingua

**D:** oferece compressão de prompt e controles para preservar tokens/dígitos. [Documentação oficial](https://github.com/microsoft/LLMLingua/blob/main/DOCUMENT.md).

**H — consequência:** preservar palavras isoladas não garante preservar a relação entre condição e consequência. Testar sentenças completas, restrições e exemplos negativos; contar tokens com o tokenizer consumidor. O compressor só entra no perfil recomendado se seu custo for compensado sem regressão indevida de qualidade. Não tratar taxas de compressão publicadas como metas garantidas para português.

## 5. Correlação entre projetos

As decisões abaixo são síntese de engenharia desta revisão; não alegações de funcionalidades ausentes em todos os projetos.

| Combinação | Insight para o objetivo | Decisão proposta |
|---|---|---|
| QMD + GraphRAG | Boa recuperação simples e síntese global atendem perguntas diferentes | Planejar rotas explícitas; grafo e resumo não precisam participar de toda consulta |
| LightRAG + Graphiti | Compartilhamento exige preservar suporte; mudança temporal exige preservar história | Separar suporte, validade e revogação; “desatualizado” e “apagado” têm semânticas distintas |
| GraphRAG + RAPTOR | Abstrações globais criam dependências além do trecho original | Manter dependências diretas e transitivas; impedir que resumo antigo sobreviva à fonte alterada |
| QMD + LLMLingua | Seleção e compressão são etapas independentes | Medir primeiro retrieval; depois benefício líquido do compressor |
| Cognee + LightRAG | Um objeto comum pode ter múltiplos donos e permissões | Reutilizar conteúdo sem herdar acesso; limitar compartilhamento ao domínio de confiança |
| Graphiti + LLMLingua | Extração e compressão podem introduzir duas perdas sucessivas | Avaliar cada transformação e também a cadeia completa contra a evidência original |
| Todos | Configuração/hardware/modelos alteram o resultado | Benchmark congelado, ablações e falhas de setup distintas de qualidade zero |

## 6. Falhas a prevenir e controles propostos

### P01 — Exclusão incompleta ou que ressuscita após reinício

Persistir OperationJournal no armazenamento autoritativo: operation_id, escopo, alvo, fase, dependências e última falha. Não copiar conteúdo integral sensível para um journal secundário sem política de retenção.

Primeiro efetivar a revogação de leitura e invalidar derivados consultáveis. Depois concluir coleta/reconstrução. A geração antiga pode continuar atendendo itens não afetados, mas jamais ignorar a revogação recente. Se não for possível provar segurança de um resumo, ocultá-lo e usar evidências sobreviventes ou sinalizar insuficiência.

Teste: encerrar o processo em cada fase, reiniciar, repetir a operação, cancelar parte de um lote e reindexar a pasta. Conferir ausência de dados revogados e preservação dos suportes ainda autorizados. Diferenciar exclusão do índice, supressão persistente de uma fonte ainda na pasta e remoção física do arquivo, que não será implícita.

### P02 — Otimização do lote destrói fonte sobrevivente

Agrupar reconstruções por derivado afetado sem assumir que todos os documentos do lote foram removidos. Publicar sobre o conjunto de suportes efetivamente válido; controle de concorrência impede ingestão e exclusão simultâneas da mesma revisão. Cache vazio não bloqueia a exclusão: descartar derivado se necessário e reconstruir depois a partir de suportes autorizados.

Teste: três fontes compartilham um fato; remover duas, cancelar após a primeira e importar outra revisão. O suporte final deve corresponder às operações efetivadas.

### P03 — Histórico vira “a última importação venceu”

Persistir observed_at e, quando sustentado, valid_from/valid_to. Fonte sem data tem validade desconhecida; mtime não prova início da validade. Correção de arquivo e mudança no mundo não são automaticamente a mesma coisa.

Teste: importar primeiro um registro recente e depois um antigo; consulta atual não regride silenciosamente. Conflitos sem regra de precedência continuam explícitos. Consultas históricas obedecem às permissões atuais.

### P04 — Vetores incompatíveis com dimensão igual

Fingerprint inclui modelo/revisão, normalização, pooling, template de entrada, dimensão e pré-processamento relevante. Mudança inicia nova geração vetorial. Não unir vetores só por terem o mesmo comprimento. Permitir fallback lexical declarado enquanto a nova geração está incompleta.

Teste: trocar entre dois modelos de mesma dimensão, interromper a migração e consultar. A saída identifica o modo usado e não mistura espaços.

### P05 — Compressão destrói a resposta

Comparar seleção extrativa, visão estruturada e compressão semântica no mesmo conjunto recuperado. Guardar spans críticos e testar relação entre sujeito, negação, número, unidade, condição e exceção. Caso falhe o validador, usar contexto extrativo dentro do orçamento ou devolver insuficiência. Não prometer detecção automática de toda perda semântica.

Teste: “permitido somente se X, exceto Y” não pode virar “permitido”; número sem unidade falha. Avaliação humana amostral continua necessária.

### P06 — Local é lento, instável ou baixa dados escondido

Manifesto de modelos explícito, provisionamento separado, limites por processo, fila limitada e prioridade para consulta. Validar schema e suporte da extração; limitar tentativas e registrar falha sem promover resultado parcial. Não escalar silenciosamente para nuvem.

Teste: modelo indisponível, JSON inválido, timeout e memória no limite. A consulta determinística continua quando seus componentes estão disponíveis; capacidades ausentes são declaradas.

### P07 — Benchmark premia erro de configuração

Pré-voo verifica corpus, documentos esperados, índice pronto, fingerprints e tokenizer. Faltar uma coleção ou evidência esperada invalida a execução, não vira zero de recall. Já uma busca válida sem resultados conta como falha de recuperação.

Avaliar português, inglês e acervo misto separadamente. Registrar denominador, cobertura, exclusões, seeds e intervalos. Não somar resultados do corpus de desenvolvimento ao teste cego.

### P08 — Contexto parcial é anunciado como resposta global

Consulta global precisa distinguir amostragem de cobertura completa. Registrar plano de recuperação, regiões consultadas, limites e lacunas; não usar “todos” quando só um top-k foi examinado. Cobertura das regiões pesquisadas não é certeza de verdade.

Teste: evidência crucial distribuída fora do top-k e orçamento insuficiente. O pacote deve informar que a visão é parcial.

## 7. Construir, integrar ou apenas comparar

| Componente | Direção |
|---|---|
| Contrato KIR, suporte, variantes, ContextPackage e lifecycle | Construir um núcleo mínimo: representam a hipótese do produto |
| Tokenização, embeddings, runtimes, banco | Integrar componentes mantidos; não reimplementar kernels |
| Retrieval local completo | Comparar com QMD antes de investir em substituição ampla |
| Grafo semântico | Comparar LightRAG; incorporar conceitos mediante revisão de licença e compatibilidade |
| Temporalidade | Adotar contrato mínimo inspirado no problema, sem importar uma plataforma inteira |
| Compressor | Adaptador opcional para experimento; não requisito obrigatório de toda consulta |
| Hierarquia global | Experimento posterior ao caminho simples, sem bloquear alpha |

**Gate construir versus compor:** se QMD mais uma camada pequena de proveniência/compilação atender aos requisitos e ao perfil de recursos, preferir composição. Só justificar novo motor completo por lacunas demonstradas em invariantes ou custo. Linguagem Rust, por si só, não prova ganho frente a uma composição existente.

Esta decisão não cancela a tese KIR: reduz a parte de infraestrutura que precisa ser reinventada. Não copiar código sem verificar a licença da revisão concreta e dos componentes transitivos.

## 8. Experimentos e ordem

| Experimento | Pergunta | Métrica/gate |
|---|---|---|
| E01: B1 × QMD × protótipo | Novo retrieval é necessário? | Qualidade e custo local com configuração congelada |
| E02: duplicação 0/50/90% | Reutilização economiza sem fundir variantes? | Bytes, trabalho evitado, falso merge, qualidade |
| E03: mudança 1/10% + lote de exclusão | Incremental continua correto sob falha? | Recomposição por derivado, invariantes P01/P02 |
| E04: datas fora de ordem | Histórico é consistente? | Casos temporais e conflitos preservados |
| E05: sem/ com bibliotecário e compressor | Camadas semânticas compensam? | Custo agregado e qualidade por transformação |
| E06: PT-BR/EN/misto | Ganhos transferem de idioma? | Métricas por idioma, sem média mascarando regressão |
| E07: orçamento e fallback | Produto degrada de forma honesta? | Tokens, memória, falhas visíveis e insuficiência |
| E08: update de modelo | Migração não mistura índices? | Fingerprints, geração e recuperação após interrupção |

Rodar E01 no M0; projetar P01–P04 no M1; E05/E06/E07 no M2; validar restore/empacotamento no M3. Começar com fixtures pequenas e expandir após validar o harness. Nenhum dos experimentos foi executado nesta revisão.

## 9. Mudanças incorporadas

[PRD vigente](02-prd.md): RAG-19 a RAG-26, operação durável, semântica temporal, fingerprint, consulta adaptativa, PT-BR, estados de degradação e pré-voo. [Plano vigente](03-plano-de-execucao.md): testes de falha, comparação QMD e gate construir/compor.

O registro acima transforma lições em mecanismos e testes planejados. A prevenção só estará demonstrada depois da implementação e execução desses testes; não se afirma que o projeto já está livre dessas falhas.


Classificação de adoção e relacionamentos upstream/downstream: [DEPENDENCIES.md](../DEPENDENCIES.md). Os projetos pesquisados não são um conjunto de dependências de runtime. A revisão v2.2 consolida essa distinção sem alterar os resultados documentais desta pesquisa.

