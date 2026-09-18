# Escopo e princípios — Mnemvia

18/09/2026. Direção vigente do produto.

## Definição

Mnemvia é um motor RAG local e open source, projetado para reutilizar conhecimento entre documentos e compilar contexto rastreável dentro de um orçamento de tokens.

Fluxo: fontes locais → detecção de mudanças → parsers/bibliotecário → representação canônica → recuperação → compilador de contexto → consumidor. O retorno ao acervo é uma proposta Markdown revisável.

## Problema central

Acervos contêm conhecimento repetido, distribuído entre fontes e sujeito a alterações. Recuperar trechos isolados pode duplicar contexto, perder relações ou ignorar exceções. A hipótese do projeto é que representação reutilizável e compilação por orçamento podem reduzir custo sem perda indevida de qualidade.

Um elemento comum não elimina suas instâncias, permissões, fontes ou diferenças. Reutilização de bytes, processamento, conhecimento e tokens são objetivos distintos, com métricas próprias.

## Limites

| Tema | Decisão |
|---|---|
| Bibliotecário, KIR e Context Compiler | Núcleo da hipótese |
| Entidades comuns, usos e variantes | Requisito inicial |
| Incrementalidade, cache e exclusão correta | Requisitos do núcleo |
| CPU, memória, disco, tokens e qualidade | Medidos em conjunto |
| Chat e geração de respostas | Consumidores substituíveis |
| Obsidian | Pasta fonte possível, não aplicativo obrigatório |
| Agentes operacionais e manutenção de hosts | Fora do núcleo |
| Projetos similares | Referências ou candidatos conforme o registro, não dependências automáticas |
| Distribuição | Open source, Apache-2.0; modelos e terceiros com termos próprios |

## Origem e evolução

A [conversa de origem](https://chatgpt.com/share/6aad4acb-f47c-83e9-abb8-e170e12a48a6) motivou bibliotecário, reutilização entre projetos e execução local. Foram consultados os trechos centrais acessíveis; o widget de pesquisa aprofundada não estava disponível na leitura. Recomendações daquela conversa são hipóteses, não resultados de desempenho.

A primeira interpretação especializou indevidamente o produto em infraestrutura. Essa direção foi substituída. O [snapshot anterior](archive/infrastructure-focus/README.md) mantém o registro histórico sem definir o projeto atual.

## Autoridade dos documentos

O [PRD](02-prd.md) define o produto; o [plano](03-plano-de-execucao.md) organiza a execução; [DEPENDENCIES.md](../DEPENDENCIES.md) registra adoção técnica. A [pesquisa comparativa](07-pesquisa-comparativa-e-prevencao.md) fundamenta decisões e distingue evidência de hipótese.

Estado: perfil determinístico M0 implementado para ingestão, busca lexical e ContextPackage local. Não há integrações externas comprovadas, consumidores confirmados, retrieval vetorial, bibliotecário semântico ou ganhos medidos.

