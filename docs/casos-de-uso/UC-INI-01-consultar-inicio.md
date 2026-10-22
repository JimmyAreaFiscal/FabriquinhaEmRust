# UC-INI-01 — Consultar o início

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Início |

## Objetivo

O Auditor vê o que espera a decisão dele, o que está em execução e a fase de cada demanda em andamento.

## Pré-condições

O serviço está no ar. O Início abre com ou sem demanda em andamento.

## Fluxo principal

1. O Auditor abre Início. O caminho `/` leva a `/home`.
2. A página lê as sessões ativas, as demandas em andamento de todos os projetos e as métricas do mês corrente.
3. Mostra a frase do mês, Aguarda você, Rodando e o board.
4. Com a aba visível, relê sozinha a cada 5 segundos. Ao voltar à aba, relê de novo. A rolagem horizontal do board fica onde estava.
5. O Auditor abre um card, uma linha ou um cartão. O gate abre a página da demanda. O restante abre o chat.

## O que a página mostra

A frase do mês reúne todos os projetos: custo, soma dos tokens, demandas registradas e concluídas, tickets criados e concluídos, pull requests abertos e mergeados. O link Métricas abre a página de métricas.

**Aguarda você.** Um card por sessão `aguardando_humano`. A demanda que a IA externa enviou e o Auditor ainda não decidiu entra sem sessão. A ordem é da mais antiga para a mais nova. A lista vazia diz "Nada aguarda você agora." Com cards, a seção mostra o número e "da mais antiga para a mais nova".

Cada card de sessão traz o motivo, o botão, `I-n`, o título, a frase do que falta, o projeto, `T-n`, a etapa, o modo e há quanto tempo. Título ausente aparece como "sem título". O modo é autônoma, interativa ou consulta.

| Espera | Botão | Abre | Frase |
|---|---|---|---|
| Falha | Responder e retomar | chat (UC-EXE-02) | o erro do ticket; sem erro gravado, "o agente falhou" |
| Demanda enviada pela IA externa | Avaliar demanda | página da demanda (UC-DEM-02) | "Demanda enviada por uma IA externa; falta aprovar" |
| Planejamento | Avaliar planejamento | página da demanda (UC-PLAN-02) | "1 ticket de desenvolvimento proposto" ou "N tickets de desenvolvimento propostos" |
| Pull request da rodada | Ver o PR | página da demanda (UC-ENC-02) | "PR #n aberto; aguarda a sua revisão no PR" |
| Conversa, ou espera sem motivo | Responder | chat (UC-EXE-02) | "O agente respondeu e espera você" |
| Diálogo | Responder | chat (UC-EXE-02) | "O agente pergunta: …"; sem título na pergunta, "O agente abriu uma pergunta" |

A sessão em aberto entra como conversa ou diálogo. O que o Auditor faz nesse chat está em UC-SES-01.

O card da demanda enviada traz a etapa demanda, o texto "IA externa" e o tempo desde o envio. O ticket é o de demanda, de origem externa, em execução e já marcado como enviado.

O `#n` é o pull request da rodada que espera o Auditor (UC-ENC-02). Sem o número, a frase fica "PR aberto; aguarda a sua revisão no PR".

**Rodando.** Uma linha por sessão `rodando`, da mais antiga para a mais nova. O número aparece mesmo em zero. A lista vazia diz "Nenhuma sessão rodando."

A linha traz o modo, `I-n`, o título, o que o agente faz, `T-n`, a etapa, o modelo e há quanto tempo. O clique abre o chat dessa sessão.

O que o agente faz é o texto do processo vivo. Sem esse texto, a consulta diz "Respondendo à sua pergunta". Nos outros modos, vale o título do ticket. Sem título de ticket no board, a linha diz "Trabalhando". Modelo ausente aparece como "padrão da etapa". Demanda fora do board entra com o título "sem título".

**Board.** Uma coluna por fase da demanda, sob as cinco etapas. A coluna de gate leva a marca "gate". Coluna vazia diz "vazia". Em cada coluna, a demanda mais nova vem primeiro. O cartão abre a página da demanda.

| Etapa | Fase | Gate |
|---|---|---|
| Demanda | `demanda` | |
| Demanda | `aprovacao_demanda` | sim |
| Planejar | `planejamento` | |
| Planejar | `aprovacao_planejamento` | sim |
| Executar | `execucao` | |
| Revisar | `revisao_demanda` | |
| Encerrar | `encerramento_demanda` | |

A fase da rodada aparece no ticket do cartão. Com a demanda em `execucao` e a rodada em `aprovacao_pr`, o cartão fica na coluna Executar (UC-ENC-02).

O contador lê "1 demanda em andamento" ou "N demandas em andamento".

O cartão traz `I-n`, o projeto, o título e o ticket não concluído mais antigo, com a etapa e o status. Os outros não concluídos entram como `(+N)`. Sem ticket fora de `concluido`, o cartão não mostra ticket. O status lê aberto, em execução, falhou, fechado ou cancelado.

| Marca | Quando |
|---|---|
| em execução · tempo | sessão `rodando` desta demanda |
| aguarda você · responder | conversa ou diálogo |
| aguarda você · falha | falha |
| aguarda você · gate da demanda | demanda enviada pela IA externa e ainda sem decisão |
| aguarda você · gate do planejamento | sessão à espera do planejamento |
| aguarda você · gate do PR | sessão à espera do pull request |
| bloqueada por I-n | dependência que ainda está no board |

A mesma marca não se repete. Várias dependências saem na ordem guardada na demanda, separadas por vírgula.

## Regras

O tempo conta os minutos cheios desde a entrada no estado. Abaixo de um minuto, "há menos de 1 min". Até 59 minutos, "há N min". A partir de uma hora, horas e minutos. A marca de execução no cartão omite "há".

O item Início da navegação conta as sessões `aguardando_humano` e só aparece quando há ao menos uma. A seção Aguarda você conta os cards, inclusive a demanda enviada sem sessão.

A demanda na fase `demanda`, ainda não enviada, fica no board e fora de Aguarda você (UC-DEM-01).

Demanda `concluida` ou cancelada fica fora do board (UC-ENC-03, UC-DEM-03). Dependência concluída ou cancelada não marca bloqueio.

## Pós-condições

Abrir e reler deixam demanda, ticket e sessão como estão. A decisão segue no caso que o card abriu.
