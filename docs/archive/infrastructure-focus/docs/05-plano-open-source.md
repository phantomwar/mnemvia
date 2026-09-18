# Opseld — plano open source

18/09/2026 • Direção atual do projeto • Licença: Apache-2.0.

Este documento substitui as premissas de distribuição fechada, planejamento centrado em uma única empresa e calendário apoiado em equipe fixa. Requisitos técnicos anteriores continuam como referência, com os ajustes abaixo. A sequência vigente está em [ROADMAP](../ROADMAP.md); não há produto executável ou repositório remoto publicado por este trabalho.

## 1. Nome escolhido

**Opseld** — diagnóstico operacional local sustentado por evidências.

Nome curto, ASCII, sem dependência de fornecedor, modelo, linguagem ou sistema operacional. O prefixo “Ops” remete a operações; a associação pretendida para o restante é evidence-linked diagnostics. Isso é uma construção de marca do projeto, não uma etimologia existente.

Usos previstos: projeto **Opseld**, futuro executável `opseld`, pacotes próprios com prefixo `opseld-`. Os identificadores são convenções propostas; ainda não existem executáveis ou pacotes publicados.

Tagline em inglês: **Local infrastructure diagnostics, backed by evidence.**

### Triagem de nomes

Consultas realizadas em 18/09/2026:

| Nome | Evidência observada | Decisão |
|---|---|---|
| Runweave | Produto de IA e projetos homônimos; pesquisa GitHub retornou nove nomes contendo o termo | Descartado por conflito próximo |
| Evidrail | Repositório existente com foco em software orientado a evidências | Descartado por proximidade de nome e tema |
| Opseld | Nenhum repositório encontrado na busca GitHub por nome; buscas web não revelaram produto de software diretamente conflitante | Escolhido para o projeto |

Referências dos conflitos: [RunWeave](https://runweave.app/), [repositório runweave](https://github.com/winor30/runweave), [Evidrail](https://github.com/dimidotdev/evidrail).

Consultas diretas para Opseld retornaram HTTP 404 nos endpoints de conta GitHub, crate, npm e PyPI. Isso significa que os registros não foram encontrados naquele momento, não que os nomes estejam reserváveis ou juridicamente livres. Buscas podem omitir projetos privados e marcas não indexadas.

Endpoints consultados: [conta GitHub](https://api.github.com/users/opseld), [crates.io](https://crates.io/api/v1/crates/opseld), [npm](https://registry.npmjs.org/opseld), [PyPI](https://pypi.org/pypi/opseld/json). A pesquisa de repositórios usou `opseld in:name` na API pública GitHub e retornou zero resultados.

Nenhum domínio foi consultado para compra ou registrado. Antes de investir em marca, fazer a busca formal nas jurisdições de interesse. Domínio não é dependência para abrir o projeto: um repositório sob a conta real do fundador já basta.

## 2. Licença escolhida e trade-off

Escolha: **Apache License 2.0**, texto oficial em [LICENSE](../LICENSE).

A prioridade assumida é facilitar adoção, contribuições e integração em infraestrutura, inclusive uso comercial. A licença possui disposições explícitas de patentes e contribuições; não exige que todos os derivados se tornem públicos. [Histórico e objetivos da licença](https://www.apache.org/licenses/license-history.html).

Essa escolha permite que terceiros ofereçam serviços pagos ou derivados fechados, obedecendo às condições aplicáveis. Não se deve vender a Apache-2.0 como proteção contra concorrência comercial.

Se a prioridade fosse obrigar reciprocidade de alterações disponibilizadas por rede, AGPL seria candidata mais alinhada; se fosse manter alterações dos arquivos cobertos abertas na distribuição, MPL seria candidata intermediária. Como o usuário definiu open source sem exigir essa reciprocidade, adoto Apache-2.0 para este projeto. A comparação e a política completa estão em [LICENSING](LICENSING.md).

O projeto seguirá licença única para código, documentação original e fixtures sintéticas originais. Dependências, modelos e dados externos conservam suas próprias condições. Não haverá restrição adicional contra empresas, provedores de nuvem ou determinados usos na licença do código.

## 3. Posicionamento e público inicial

**Opseld ajuda operadores a investigar serviços locais com evidências da infraestrutura e conhecimento do ambiente, sem depender de uma API paga.**

Público inicial: mantenedores de serviços self-hosted, administradores Linux, pequenas equipes de infraestrutura e laboratórios que precisam conectar runbooks a observações. A hipótese deve ser validada com usuários externos, não apenas no ambiente do fundador.

Open source muda a prioridade de onboarding: o primeiro usuário precisa conseguir reproduzir uma demonstração sem fornecer segredos, cadastrar a infraestrutura inteira ou contratar suporte. Depois ele conecta suas próprias fontes privadas localmente. Essas fontes nunca viram públicas apenas por usar o produto.

Não prometer “SRE autônomo”, “causa raiz garantida” ou “compatível com qualquer modelo”. Informar exatamente quais casos, versões e configurações foram testados.

## 4. Princípios do produto aberto

1. Instalação local sem conta obrigatória na infraestrutura do projeto.
2. Sem telemetria de uso por padrão; qualquer coleta futura exige especificação pública, consentimento e opção de desligar.
3. Sem API paga obrigatória para a jornada básica.
4. Não esconder autorização, auditoria, redaction ou restauração atrás de edição paga.
5. Exemplos públicos sintéticos e redistribuíveis; dados privados ficam sob controle do operador.
6. Formatos documentados, exportação e reconstrução do conhecimento sem dependência de serviço do mantenedor.
7. Modelos e coletores substituíveis; integração ampla somente após evidência de demanda e capacidade de manutenção.
8. Releases identificam claramente recursos planejados, experimentais e efetivamente testados.

Esses princípios orientam a distribuição oficial. Uma licença permissiva não obriga todo fork a manter as mesmas escolhas de produto.

## 5. Roadmap orientado à capacidade de mantenedores

| Marco | Valor para quem está de fora | Condição para avançar |
|---|---|---|
| M0 — fundação pública | Saber o que existe, como contribuir e sob quais regras | Identidade do mantenedor, direitos de publicação e canais reais estabelecidos |
| M1 — demo offline | Reproduzir diagnóstico e comparar com regras sem acessar produção | Fixtures, comando reproduzível e resultados medidos |
| M2 — alpha Linux read-only | Investigar um serviço real com escopo restrito | Políticas, redaction, auditoria, recuperação e reporte privado verificados |
| M3 — beta operacional | Usar em ambientes diferentes e atualizar com segurança | Compatibilidade documentada, migração e feedback externo |
| M4 — experimento de ação governada | Testar uma ação limitada em homologação | Aprovação vinculada, reconciliação e recuperação demonstradas |

Windows e Proxmox são trilhas opcionais por demanda; não uma obrigação simultânea de v1. Execução limitada depende dos controles relevantes, não de terminar todos os conectores. LoRA, grafo dedicado e SaaS permanecem experimentos posteriores.

### Esforço e sustentabilidade de agenda

O experimento de validação estimado em 6–8 pessoa-semanas é cenário de trabalho, não data de release. Com uma pessoa disponibilizando dez horas semanais ao projeto, isso corresponde aproximadamente a 24–32 semanas se uma pessoa-semana representar quarenta horas e não houver colaboradores. Com duas pessoas dedicadas, o calendário pode diminuir, respeitando dependências.

As faixas anteriores de 10–15 semanas para MVP pressupunham três FTE e não são metas públicas do open source. Reestimar após saber disponibilidade real, aproveitamento de componentes e tamanho do protótipo. Contribuições voluntárias são ganho potencial, não capacidade contratada no cronograma.

Usar esforço completo por pacote, incluindo revisão e testes. Converter em calendário pela disponibilidade efetiva; não descontar novamente como indisponibilidade o mesmo trabalho já incluído na estimativa.

## 6. Demo que facilite adoção

Antes do piloto online, preparar pelo menos cinco incidentes sintéticos: serviço parado, volume cheio, falha de conectividade, evidência contraditória e causa não observável com os dados disponíveis. Cada fixture contém escopo, timestamps, fontes e resultado esperado.

O baseline determinístico roda sem pesos. A opção com LLM explica download/provisionamento separado, tamanho e requisitos medidos; não baixa pesos silenciosamente. A jornada deve apresentar evidências, hipóteses e próximo teste, com tempos de coleta visíveis.

Comparar regras, retrieval simples, contexto com relações e HolmesGPT restrito. Publicar limitações e resultados negativos. Não declarar superioridade a partir de testes com modelos, evidências ou budgets diferentes sem explicitar a diferença.

Uma pessoa externa deve conseguir repetir a demo a partir da documentação. Meta de onboarding após build e provisionamento: até quinze minutos para executar o caso incluído; downloads e compilação são medidos separadamente. É critério de produto proposto, não resultado atual.

## 7. Ajustes incorporados à direção do PRD

- Admission control considera trabalho estimado, não apenas quantidade de itens na fila.
- Pacote mínimo de evidências sanitizadas acompanha o incidente, condicionado à retenção permitida; exclusão prevalece sobre replay.
- Autorização atual é revalidada na entrega de resposta e exportação, mesmo com snapshot antigo fixado.
- Replay usa somente informação disponível no instante simulado, sem postmortem futuro.
- Qualidade mede suporte real das citações e utilidade da próxima decisão, além de top-1/top-3.
- Deployment/commit/digest deve estar ligado ao serviço quando o caso avaliado for pós-deploy.
- Coleta existente é avaliada antes de agente próprio, preservando controle de escopo e dados.
- Ação futura promete deduplicação e reconciliação, não uma garantia genérica de exactly-once.

### Requisitos próprios do open source

| ID | Requisito | Aceite |
|---|---|---|
| OSS-01 | Licenciamento e origem | Texto oficial, licenças de componentes registradas e contribuições com DCO |
| OSS-02 | Instalação/demonstração reproduzível | Execução documentada em ambiente limpo, sem dados privados ou API paga obrigatória |
| OSS-03 | Transparência | Status real, roadmap, limitações e decisões acessíveis |
| OSS-04 | Privacidade local | Nenhum envio não consentido de dados de uso ou incidentes |
| OSS-05 | Contribuição modular | Pequeno caso de teste ou adapter pode ser revisado sem redesenhar todo o sistema |
| OSS-06 | Segurança de distribuição | Reporte privado real, verificação de artefatos e secrets isolados de PRs não confiáveis |
| OSS-07 | Portabilidade de dados | Export/import versionados e exemplo público de reconstrução |
| OSS-08 | Manutenibilidade | Dono e plataforma de teste definidos para cada integração suportada |

## 8. Repositório e contribuição

Estrutura já preparada: README em inglês/português, LICENSE, DCO, CONTRIBUTING, GOVERNANCE, SECURITY, CODE_OF_CONDUCT, ROADMAP e documentos de decisão. Links internos são relativos para funcionar fora da máquina do autor.

Estrutura de implementação futura, a criar quando houver conteúdo: domínio/protocolo, adapters de coleta, inferência, interface e fixtures/evals. Um monorepo é suficiente inicialmente. Não criar muitos pacotes vazios só para simular maturidade.

O projeto aceita DCO, sem cessão de copyright ou CLA separado no modelo inicial. Manter alterações focadas e revisar origem de material gerado por IA como qualquer outra contribuição. [Developer Certificate of Origin](https://developercertificate.org/).

Boas primeiras contribuições: corrigir documentação, traduzir um fluxo, criar fixture sintética com resultado esperado, testar uma instalação e documentar uma incompatibilidade. PKI, autorização, executor e migrações críticas não são tarefas iniciais sem acompanhamento.

Não abrir issues de trabalho fictícias nem prometer atendimento. Antes de convidar contribuições, o mantenedor define quanto consegue revisar. Inglês é o idioma principal de interfaces e documentos públicos centrais; português continua aceito.

## 9. Governança e confiança

Começar com governança simples liderada pelo fundador. Publicar o responsável real e a área de cada mantenedor; não inventar uma fundação ou equipe. Discussões significativas usam proposta escrita e decisão registrada. Critérios para novos mantenedores estão em [GOVERNANCE](../GOVERNANCE.md).

No crescimento, buscar duas pessoas capazes de publicar releases e responder a incidentes. Isso é meta de resiliência, não condição artificial para iniciar a documentação ou alegação de equipe existente.

Segurança e conduta precisam de contatos reais. Como ainda não existe remoto ou identidade de hosting definida neste trabalho, as políticas informam a ausência de canal privado. Ativá-lo e testá-lo é bloqueio de alpha pública, não pedido de confirmação para continuar o planejamento.

## 10. CI, releases e cadeia de fornecimento

Antes da primeira release de código: configurar testes, formatação/lint relevantes, verificação de origem/licenças, DCO e varredura de segredos. Documentação pode ter validação de links. Não há workflow configurado nesta entrega.

Separar testes de pull request de publicação de artefatos. PR externo não recebe credenciais. Fixar versões das ferramentas e ações de automação quando escolhidas; registrar permissões mínimas. Produzir checksums, inventário de dependências e proveniência de build compatível com a infraestrutura usada.

Não publicar pacotes vazios apenas para ocupar nomes. Registrar o nome sob conta controlada pelo projeto quando houver artefato e decisão de hosting. Pesos e dados privados não entram em releases, imagens de exemplo ou anexos de issues.

Pré-releases 0.x declaram experimentalidade e mudanças incompatíveis. Sem promessa de LTS no início. A versão 1.0 exige contratos e upgrade/restore documentados, avaliação reproduzível e capacidade de manutenção.

## 11. Financiamento e métricas

Começar sem dependência de receita. Depois considerar patrocínio, grants, suporte, implantação assistida e desenvolvimento de conectores. Hosting opcional pode financiar manutenção, mas não deve virar requisito para usar a distribuição local.

Medir instalações reproduzidas, tempo até primeira evidência útil, ambientes externos ativos, issues resolvidas com qualidade, quantidade de integrações realmente mantidas e concentração do trabalho em uma pessoa. Não otimizar o roadmap só por estrelas.

Cada integração adicionada tem custo contínuo de testes, credenciais, versões e suporte. Exigir demanda concreta e mantenedor comprometido antes de prometer suporte oficial.

## 12. Próximos passos concretos

1. Escolher a conta/organização real que hospedará o projeto e registrar o mantenedor inicial; a escolha não foi inferida a partir do nome da pasta local.
2. Revisar direitos sobre fontes externas e confirmar que nada privado será incluído. Os anexos originais não foram copiados para o repositório.
3. Publicar a base documental e ativar contatos privados reais para segurança e conduta.
4. Criar o formato de pacote de evidências e a primeira fixture sintética.
5. Implementar baseline sem IA; depois ligar modelo local e comparar resultados.
6. Convidar poucos operadores externos para reproduzir o experimento e replanejar a alpha com sua experiência.

**Estado desta entrega:** nome e licença escolhidos; estrutura documental criada localmente; planos adaptados. Não foram criados remoto Git, organização, domínio, pacote, marca ou release.
