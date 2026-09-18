# Documentação do Mnemvia

Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

**Estado: implementação determinística M0.** O CLI local ingere Markdown/texto e `composer.json`, preserva revisões, fornece busca lexical/ContextPackage, inspeciona proveniência/fatos/estado do índice, marca operações interrompidas para revisão e verifica invariantes em modo somente leitura. Não há modelo, vetor, serviço externo, API de rede ou integração implementada.

## Comece aqui

| Documento | Papel |
|---|---|
| [README em inglês](../README.md) / [português](../README.pt-BR.md) | Descrição pública, fluxo e estado do projeto |
| [Dependências e projetos relacionados](../DEPENDENCIES.md) | Adoção, candidatos, referências e consumidores |
| [PRD v2.2](02-prd.md) | Requisitos e critérios de aceite |
| [Execução v2.2](03-plano-de-execucao.md) | Etapas, experimentos e dependências entre entregas |
| [Roadmap](../ROADMAP.md) | Visão pública dos marcos |
| [Escopo e princípios](06-foco-original.md) | Objetivo e limites |
| [Pesquisa comparativa](07-pesquisa-comparativa-e-prevencao.md) | Sete referências, correlações e prevenção de falhas |
| [Análise correlacionada](01-analise-correlacionada.md) | Síntese e acesso à evolução da proposta |
| [Revisão crítica](04-revisao-critica-e-recomendacoes.md) | Recomendações vigentes |

## Open source e participação

[Plano open source](05-plano-open-source.md), [decisão de nome](adr/0002-project-renaming.md), [licenciamento](LICENSING.md), [contribuição](../CONTRIBUTING.md), [governança](../GOVERNANCE.md), [segurança](../SECURITY.md) e [código de conduta](../CODE_OF_CONDUCT.md).

O PRD é a autoridade de produto. O registro de dependências descreve decisões de adoção; manifestos e lockfiles serão a autoridade do conjunto instalado quando existirem. Pesquisa não constitui aprovação de dependência.

## Histórico

O [arquivo da proposta anterior](archive/README.md) preserva a direção superada centrada em infraestrutura. Seus requisitos, nome e prioridades não são vigentes. Documentos de entrada e orientações atuais devem referenciar o PRD v2.2 ou posterior, não o snapshot.

## Manutenção

Alterações de escopo atualizam README, PRD e plano juntos. Adoção de um componente atualiza DEPENDENCIES.md, manifestos/lockfiles e notas de compatibilidade. Resultados publicados precisam apontar execução reproduzível. Preservar a consistência entre inglês e português sem anunciar funcionalidades planejadas como prontas.

