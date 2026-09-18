# Análise correlacionada — IA local para conhecimento e diagnóstico operacional

Data: 18/09/2026. Versão: 1.0. Natureza: análise documental com verificação pontual de premissas técnicas.

## 1. Conclusão

O projeto tem uma tese consistente: organizar conhecimento empresarial e evidências operacionais para que uma IA local produza diagnósticos verificáveis. O componente diferenciador é o pipeline de conhecimento, contexto e autorização; o modelo de linguagem é substituível.

Recomendo iniciar por uma pergunta delimitada: **“Por que este serviço está indisponível neste host Linux?”** O primeiro produto deve coletar evidências autorizadas, mostrar hipóteses com fontes e registrar o desfecho humano. Windows, Proxmox, recuperação vetorial e mutações entram por etapas.

O blueprint traz a arquitetura; a revisão HTML acrescenta controles operacionais úteis. Entretanto, a revisão não deve ser adotada integralmente: sua principal acusação sobre MoE é incorreta para os modelos 4B/9B analisados, e a conclusão sobre Cognee/Postgres omite restrições documentadas.

## 2. Fontes e método

| ID | Documento | Função |
|---|---|---|
| D1 | `deep-research-report.md` (anexo externo, não redistribuído) | Visão, arquitetura, stack, evolução e recomendações |
| D2 | `file (6).html` (anexo externo, não redistribuído) | Revisão crítica de D1, lacunas e reestimativa |

Integridade dos arquivos recebidos, SHA-256:

- D1: `87B9885D6A9F6B3278F0C54E01C57F9DCC0AC5C1A185C621C91BE5CA1C221CA9`.
- D2: `7BFEB27DFCF33B3EF4052E9ACA5E03D25F73B72FAB3248DF579901E00AB05D7D`.

Foram correlacionados problema, fluxo de conhecimento, dados, modelos, agentes, segurança, métricas e planejamento. As afirmações externas que poderiam mudar a decisão de arquitetura foram confrontadas com documentação primária e Context7. Não foi executado benchmark, teste de hardware ou validação de infraestrutura da empresa.

Expressões como “ação obrigatória”, “Astra deve” e “não escrever código” foram tratadas como recomendações de seus autores. Elas não autorizam instalação, treinamento, acesso a servidores ou execução de comandos neste trabalho. O pedido atendido é produzir análise, PRD e plano.

## 3. Correções técnicas relevantes

### 3.1 Qwen3.5: atenção híbrida não significa MoE

Os model cards possuem uma apresentação genérica da família mencionando MoE. Porém, a documentação de implementação distingue variantes densas das variantes MoE, e as configurações específicas de 4B/9B usam `qwen3_5_text`, FFN e atenção linear/completa. A classificação desses dois modelos como MoE em D2 é uma generalização indevida. Eles são multimodais, com backbone textual denso e atenção híbrida. [Documentação Qwen3.5](https://huggingface.co/docs/transformers/model_doc/qwen3_5), [configuração 4B](https://huggingface.co/Qwen/Qwen3.5-4B/blob/main/config.json), [configuração 9B](https://huggingface.co/Qwen/Qwen3.5-9B/blob/main/config.json).

Consequência: retirar a justificativa de “experts residentes” do dimensionamento. Manter a medição de memória real, contexto, latência e qualidade. Mesmo para MoE, uma conta baseada em **todos** os parâmetros não se torna inválida simplesmente por haver experts; o problema seria usar apenas parâmetros ativos ou confundir pesos ideais com consumo total.

D1 já reconhece que sua conta Q4 é idealizada. O problema é transformar essa conta em confiança sobre uma máquina ainda não medida. Tampouco é possível concluir velocidade relativa de 4B e 9B apenas pelo tamanho.

### 3.2 Compatibilidade de runtime: evidência documental existe

A documentação de llama.cpp consultada lista Qwen3.5-4B/9B em sua matriz OpenVINO, com diferenças por modo/backend. Isso refuta tratar ausência de suporte como fato estabelecido. Não certifica o binário, backend CPU, quantização ou template que serão usados no piloto. [Matriz oficial](https://github.com/ggml-org/llama.cpp/blob/master/docs/backend/OPENVINO.md).

Decisão: executar um teste de compatibilidade com artefatos e versões fixados. Se falhar, trocar o candidato ou runtime pelo adaptador. Essa investigação bloqueia a escolha da configuração de inferência, não o desenho da KIR, das permissões ou das fixtures.

### 3.3 Cognee/Postgres: possibilidade não equivale a padrão de produção

O README atual informa uma opção de memória em Postgres, mas identifica o armazenamento de grafo aberto como demonstração e remete a uma oferta licenciada para produção. D2 não sustenta sua descrição de Postgres único como padrão irrestrito. Além disso, D1 associa backend de grafo dedicado explicitamente ao Graphiti, não faz essa exigência explícita na seção Cognee. [README oficial do Cognee](https://github.com/topoteretes/cognee#run-the-whole-memory-layer-on-postgres).

Decisão: comparar uma versão fixada, incluindo licença, backend, dependências e comportamento local. Nenhum desses frameworks é dependência obrigatória do MVP.

### 3.4 Outros ajustes de interpretação

- **Citações:** D1 possui links oficiais no final, mas os marcadores internos `turn…` não permitem auditar cada afirmação fora da conversa original. É rastreabilidade parcial, não ausência total de referências.
- **Read-only:** leitura pode exigir privilégio e expor segredos. Isso não torna a operação uma mutação, mas exige controles independentes de privilégio e sensibilidade.
- **Proveniência:** uma inferência pode ter proveniência completa de suas evidências e ainda estar errada. Cobertura de fontes e acurácia são métricas diferentes.
- **Framework de agentes:** uma máquina de estados específica para diagnóstico é compatível com não construir um framework genérico. O limite precisa estar no escopo.
- **Restart:** reiniciar serviço pode interromper sessões ou perder estado volátil. Não deve ser classificado universalmente como reversível.
- **mTLS:** autentica o agente e protege transporte; não comprova que evidências de um host comprometido são verdadeiras.
- **Versões de sqlite-vec e números de estrelas:** não foram revalidados e não fundamentam a decisão. A versão candidata deve ser inspecionada na futura POC.

## 4. Matriz de correlação e decisão

| Tema | D1 | D2 | Decisão consolidada | Rastreio |
|---|---|---|---|---|
| Problema inicial | Plataforma ampla | Uma indisponibilidade Linux | MVP de um host e serviços cadastrados | RF-01, RF-07 |
| Conhecimento humano | Obsidian/Git | Mantém | Markdown/Git como fonte; Obsidian opcional | RF-02 |
| KIR | Entidades, fatos e relações | Mantém | Schema versionado mínimo, com proveniência | RF-03 |
| Deduplicação | Pipeline semântico completo | Considera subestimado | Match exato no MVP; ambíguos separados | RF-04 |
| Contexto | Grafo + vetor + FTS | Orçamento de tokens | FTS + relações no MVP; vetor condicionado a ganho | RF-06 |
| Hardware/modelo | 4B inicial, 9B candidato | Questiona viabilidade por MoE | Corrigir MoE; medir ambos sem compromisso antecipado | RNF-02, G0 |
| Credenciais | Proteção de dados | Falta backend de secrets | Referência opaca, acesso restrito e rotação | RF-10 |
| Logs | Redaction no pipeline | Observações podem vazar | Redaction antes de transporte/persistência | RF-05, RF-10 |
| Temporalidade | TTL e estados | Contestação e SLA | Separar validade, confiança e desired/observed | RF-03, RF-12 |
| Tools | Lista extensa | Aproximadamente cinco | Cinco tools tipadas e limitadas | RF-05 |
| Auditoria | Todas as chamadas | Reforça riscos | Eventos duráveis e comportamento fail-closed | RF-09 |
| UX | Principalmente v1 | Escopo reduzido | UI mínima no MVP para validar utilidade | RF-08 |
| Multiplataforma | Linux/Windows/Proxmox | Esforço subestimado | Linux primeiro; conectores entregues separadamente | RF-13 |
| Execução | v2 com approvals/rollback | Reforça controle | v2 limitada; recuperação explicitamente classificada | RF-15 |
| Treinamento | LoRA no v2 | Não resolve dataset | Experimento opcional após evidência de necessidade | RF-17 |
| Retenção/fila | Pouco definido | Lacunas explícitas | Políticas desde MVP | RNF-03, RNF-05 |
| Estimativa | 40–64 pessoa-semanas | 74–118 pessoa-semanas | Nova decomposição, sem adotar multiplicador como prova | Plano, seção 3 |

## 5. Dependências causais

Uma identificação incorreta de entidade pode ligar evidências do ambiente de homologação a produção; isso contamina o contexto, o diagnóstico e uma eventual ação. Por isso, identidade, autorização e proveniência antecedem recuperação semântica sofisticada.

TTL e fila também se relacionam: uma observação válida no início do incidente pode expirar enquanto o modelo espera. O compilador deve reavaliar freshness antes da inferência, e a execução futura deve verificar pré-condições novamente no agente.

Redaction precisa ocorrer antes de indexação e auditoria: proteger apenas o prompt deixa cópias em banco, embeddings, traces e backups. Um hash de senha de baixa entropia também pode facilitar ataques; registrar identificadores de credenciais, não seus valores ou hashes simples.

Uma fonte desejada diferente de uma observação não é necessariamente contradição. “Deve executar versão A” e “executa versão B” podem ser simultaneamente verdadeiros e caracterizar drift. Contestar automaticamente ambos destruiria informação útil.

## 6. Decisões propostas e limites

Adotar como base do PRD: Core e agente próprios em Rust; runtime separado; SQLite com relações e busca textual; inferência local; fontes autorizadas; nenhum comando de shell livre; observabilidade e avaliação desde o início.

Manter condicionais: modelo, quantização, runtime exato, hardware adicional, backend vetorial, framework de memória, integrações externas e LoRA. Não há evidência para fixá-los definitivamente nesta análise.

O esforço foi decomposto no plano em 4–6 pessoa-semanas de descoberta, 16–24 de MVP, 28–44 de expansão e 24–40 de execução limitada: 72–114 no total. Trata-se de estimativa para o escopo novo, não da confirmação das estimativas dos anexos.

## 7. Documentos de execução

- [PRD detalhado](02-prd.md): objetivos, requisitos, dados, experiência, segurança e critérios de aceite.
- [Plano de execução](03-plano-de-execucao.md): pacotes de trabalho, dependências, estimativas, gates e próximos passos.

As fontes online foram consultadas em 18/09/2026 e apontam para páginas mutáveis. Antes da implementação, registrar os commits/releases e hashes exatos dos artefatos escolhidos. A verificação desta análise é documental; os resultados operacionais permanecem pendentes.
