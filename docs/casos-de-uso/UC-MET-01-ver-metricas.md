# UC-MET-01 — Ver as métricas

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página Métricas |

## Objetivo

O Auditor lê um período de meses: o mês em foco contra a média dos meses anteriores, por etapa, por modelo e por demanda. A página só lê. A métrica não barra o ticket.

## Pré-condições

O Auditor está no web. A página lê sessões, decisões e o estado já gravados. Não exige demanda numa fase.

## Fluxo principal

1. O Auditor abre Métricas.
2. A página lê os projetos, os meses do período e as demandas com execução iniciada nele. O período inicial cobre os 11 meses anteriores e o mês corrente. O foco é o mês corrente. O card selecionado é o custo.
3. O Auditor troca o projeto, o De ou o Até. A página lê de novo. O foco permanece quando o período ainda o contém. Fora do período, o foco vai para o Até.
4. O Auditor clica um mês na régua ou na evolução. A média passa a usar só os meses do período anteriores a esse. Essa troca não lê de novo.
5. O Auditor clica um card para ver a evolução daquele número, troca o gráfico pela tabela, ou ordena as demandas pelo cabeçalho. Nenhuma dessas trocas lê de novo.
6. O botão últimos 12 meses repõe o De, o Até e o foco do passo 2 e lê de novo. O card, a ordem e a tabela ficam como estão.

A página não se atualiza sozinha.

## O que a página mostra

Os filtros são o projeto, ou todos os projetos, o De e o Até. Cada seletor lista do mês corrente até 35 meses antes. A frase compara o mês em foco com a média dos anteriores do período, no rótulo curto do mês, como "comparando out/26 com a média de 11 meses (nov/25 a set/26)". Com um mês anterior só, cita esse mês. No primeiro mês do período, diz que ele é o primeiro e que não há média para comparar.

A régua tem um botão por mês do período. A altura da barra é o custo daquele mês. O botão pressionado é o mês em foco.

Seis cards do mês em foco. Cada um mostra o valor, a média dos meses anteriores e a variação.

| Card | Valor do mês | Ao lado do valor |
|---|---|---|
| Custo | Soma do custo das sessões, em US$. | — |
| Tokens | Soma de entrada, saída, cache de leitura e cache de escrita. | — |
| Demandas concluídas | Demandas concluídas. | De quantas foram registradas. |
| Tickets concluídos | Tickets concluídos. | De quantos foram criados. |
| PRs mergeados | Pull requests mergeados. | De quantos foram abertos. |
| Nota média dos PRs | Média simples de cada nota gravada no mês, de 1 a 5, na aprovação e na reprovação (UC-ENC-02). | "de 5", e quantas notas. Mês sem nota mostra "— de 5 · 0 notas". |

No primeiro mês do período, o card diz "sem meses anteriores" e não mostra variação. Com meses anteriores e número ausente, o card diz "média —". A variação é o percentual inteiro do valor contra a média, com ▲, ▼ ou =. Fica vazia quando o valor falta, a média falta ou a média é zero. Mês sem nota fica fora da média da nota. A média de uma contagem sai com uma casa quando não é inteira.

A evolução é uma coluna por mês do card escolhido, do De ao Até. O mês em foco vem marcado. Os meses anteriores entram na média. Os posteriores aparecem e ficam fora dela. Uma linha marca essa média. Passar o mouse mostra o valor, o número ao lado do card e, no mês em foco, a média. Clicar a coluna muda o foco. Ver tabela troca o gráfico pela mesma série, mês a mês. Ver gráfico volta o gráfico.

Por etapa, o mês em foco contra a média. A ordem é planejar, implementar, revisar a rodada, encerrar a rodada, sessão em aberto e revisar a demanda. As colunas são o custo do mês, a média, a variação, e, no formato mês / média, tokens, execuções, falhas, reprovações e duração média. Etapa sem execução deixa a duração em "—" e fica fora da média da duração.

Por modelo, só o mês em foco. Cada provedor e modelo, do maior custo para o menor. No empate de custo, do maior número de tokens para o menor. Mostra o custo, como o pi calculou, e os tokens de cache, entrada e saída. Sem modelo, a página diz que nenhum modelo foi usado naquele mês.

Por demanda, as demandas com execução iniciada no período. A linha tem `I-n` com link para a página da demanda, o título, o projeto só quando o filtro é todos, a fase, o custo, os tokens, as execuções, as falhas, as reprovações, a duração e a nota dos pull requests. Sem título, a página diz "sem título". Sem nota, "—". Sem demanda no período, diz que nenhuma demanda teve execução no período. O rodapé soma custo, tokens, execuções, falhas, reprovações e duração. A nota não entra no rodapé.

A fase aparece como `demanda`, `aprovacao_demanda`, `planejamento`, `aprovacao_planejamento`, `execucao`, `revisao_demanda`, `encerramento_demanda`, `concluida`, `arquivada` ou `cancelada`.

A mesma leitura do mês, sem nota e sem média, alimenta outras duas superfícies. No projeto, cinco cards do mês corrente só dele: custo com as chamadas de LLM, tokens de entrada, saída e cache, demandas registradas e concluídas, tickets criados e concluídos, pull requests abertos e mergeados (UC-PRJ-01). No Início, a frase reúne todos os projetos (UC-INI-01).

## Regras

| Número | Entra quando |
|---|---|
| Custo, tokens e modelos do mês | A execução que gerou o uso começou no mês. O mês é o do fuso do serviço. A execução que vira o mês conta inteira no mês em que começou. |
| Demandas registradas e concluídas | A data de registro ou de conclusão cai no mês. Demanda arquivada conta. |
| Tickets criados e concluídos | A data de criação ou de conclusão cai no mês. Ticket removido não conta. |
| Pull requests abertos | A primeira evidência `pr_aberto` de cada rodada é do mês. Atualizar o mesmo pull request não conta de novo. |
| Pull requests mergeados | A aprovação do pull request da rodada é do mês (UC-ENC-02). |
| Nota do mês | Cada nota gravada no mês entra, na aprovação e na reprovação (UC-ENC-02). A média do mês é a média simples dessas notas. |
| Execuções, falhas e duração da etapa | A execução autônoma ou interativa começou no mês. Falha é execução com resultado falha. A duração soma só as terminadas. |
| Reprovações da etapa | A decisão reprovada cai no mês da decisão e na etapa do gate. |
| Linha da demanda | A execução começou no período. Fase e nota do pull request são as de agora. Os demais números são só do período. A duração da linha é a soma. |

Custo, tokens, execuções, falhas e reprovações são os nomes que UC-ENC-01 grava no relatório de produtividade e que UC-ENC-02 mostra no gate. Aqui a soma é do mês. Tokens e custo de cada sessão continuam os de UC-PLAN-01, UC-IMPL-01, UC-REV-01, UC-REV-02, UC-ENC-01, UC-ENC-03 e UC-SES-01.

A consulta entra no custo, nos tokens e nos modelos do mês, e na linha da demanda. A tabela por etapa deixa a consulta de fora.

A duração média da etapa divide a soma das execuções terminadas pelo número de execuções dela. Execução sem término entra na contagem e não entra na soma.

| Gate da decisão reprovada | Etapa |
|---|---|
| Planejamento (UC-PLAN-02) | planejar |
| Pull request da rodada (UC-ENC-02) | encerrar a rodada |

Implementar, revisar a rodada, sessão em aberto e revisar a demanda não recebem reprovação deste gate. A revisão da rodada e a da demanda decidem no próprio caso.

A média de um card é a média simples dos meses do período anteriores ao foco. A média da nota entre meses é a média dessas médias mensais. Um mês com uma nota pesa como um mês com várias.

O período tem no máximo 24 meses. De não fica depois de Até. A opção fora disso fica desabilitada. O mês é `AAAA-MM`.

O mesmo cabeçalho inverte a ordem. Outro cabeçalho começa crescente em demanda, título, projeto e fase, e decrescente nos números. A fase crescente segue a ordem em que a página lista as fases, com Concluída, Arquivada e Cancelada no fim. Demanda sem nota fica antes das que têm, na ordem crescente da nota.

A nota da coluna é a média das notas já gravadas nas rodadas desta demanda, aprovação e reprovação, de qualquer data (UC-ENC-02). Várias rodadas entram todas na média.

A página não grava decisão e não muda fase.

## Pós-condições

Nada no processo mudou. Nenhuma demanda, rodada ou ticket foi barrado. O período, o foco, o card, a ordem e a escolha entre gráfico e tabela permanecem na página até o Auditor sair ou mudar.
