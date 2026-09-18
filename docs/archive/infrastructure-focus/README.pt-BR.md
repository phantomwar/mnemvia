# Opseld

**Diagnóstico local de infraestrutura, sustentado por evidências.**

[English](README.md) · [Roadmap](ROADMAP.md) · [Contribuições](CONTRIBUTING.md) · [Governança](GOVERNANCE.md) · [Licença](LICENSE)

Opseld é um projeto open source para relacionar conhecimento operacional, observações da infraestrutura e modelos de linguagem locais. O objetivo é ajudar a investigar incidentes com fontes rastreáveis, incertezas explícitas e acesso limitado às ferramentas.

**Estágio atual: planejamento. Ainda não existe aplicação executável ou release publicada.** Os recursos descritos são planejados.

## Primeira entrega

Uma demonstração offline baseada em incidentes sintéticos, comparando regras determinísticas, recuperação textual simples e contexto organizado por evidências. O baseline não exigirá acesso a produção, conta de nuvem ou API paga. Execuções com LLM dependerão de pesos compatíveis provisionados separadamente.

Depois: piloto Linux somente leitura. Windows, Proxmox, recuperação avançada e ações governadas serão marcos posteriores e independentes.

## Princípios

- Separar fontes, observações e hipóteses da IA.
- Vincular afirmações a evidências e reconhecer o que não se sabe.
- Funcionar localmente, mantendo regras e evidências úteis sem o modelo.
- Aplicar autorização fora da LLM.
- Não incluir segredos, dados privados de incidentes ou pesos no repositório.
- Avaliar coletores existentes antes de reconstruir integrações.
- Não disponibilizar shell irrestrito ou ações destrutivas automáticas.

Rust e SQLite são candidatos iniciais. O experimento validará o escopo antes da construção de uma plataforma ampla.

## Como contribuir agora

Revisar o desenho, propor casos sintéticos, encontrar premissas incorretas e melhorar o protocolo de avaliação. Ainda não há comandos de instalação ou build. Issues e pull requests poderão ser usadas quando o repositório for publicado; português e inglês são aceitos.

Leia [CONTRIBUTING](CONTRIBUTING.md) e o [status do canal de segurança](SECURITY.md). Não envie logs reais, credenciais ou informações pessoais.

## Planejamento

O [plano open source](docs/05-plano-open-source.md) define nome, licença, público, contribuição, sustentabilidade e critérios de release. O [roadmap](ROADMAP.md) é a referência de sequência. As estimativas anteriores de equipe contratada são cenários de esforço, não promessas de calendário para a comunidade.

## Licença

Apache-2.0 para materiais originais do projeto, salvo indicação específica. Permite uso comercial e derivados fechados, com cumprimento das condições da licença; não exige abertura de alterações apenas por hospedar um serviço. Dependências, pesos de modelos e dados de terceiros conservam suas licenças. Veja a [política de licenciamento](docs/LICENSING.md).
