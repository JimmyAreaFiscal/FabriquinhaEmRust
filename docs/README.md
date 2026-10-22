# Casos de uso

Cada linha aponta um caso: caminho, nome e objetivo. O fluxo, as regras e as evidências estão no arquivo.

## Início

- [`docs/casos-de-uso/UC-INI-01-consultar-inicio.md`](casos-de-uso/UC-INI-01-consultar-inicio.md) — **UC-INI-01 — Consultar o início.** O Auditor vê o que espera a decisão dele, o que está em execução e a fase de cada demanda em andamento.

## Configuração

- [`docs/casos-de-uso/UC-CFG-01-configurar-agendador.md`](casos-de-uso/UC-CFG-01-configurar-agendador.md) — **UC-CFG-01 — Configurar o agendador.** O Auditor decide quando a Fabriquinha procura trabalho sozinha e por quanto tempo uma execução pode durar.
- [`docs/casos-de-uso/UC-CFG-02-cadastrar-projeto.md`](casos-de-uso/UC-CFG-02-cadastrar-projeto.md) — **UC-CFG-02 — Cadastrar um projeto.** O Auditor registra um repositório, escolhe as linguagens e, para cada uma, o caminho do código e o comando de teste.
- [`docs/casos-de-uso/UC-CFG-03-editar-projeto.md`](casos-de-uso/UC-CFG-03-editar-projeto.md) — **UC-CFG-03 — Editar um projeto.** O Auditor altera a configuração de um projeto já cadastrado, inclusive linguagens, comandos de teste, fitness functions e o effort de cada etapa.
- [`docs/casos-de-uso/UC-CFG-04-remover-projeto.md`](casos-de-uso/UC-CFG-04-remover-projeto.md) — **UC-CFG-04 — Remover um projeto.** O Auditor tira um projeto da Fabriquinha, junto com o processo, os custos e os arquivos que ela guardou para ele.
- [`docs/casos-de-uso/UC-CFG-05-entrar-no-servico.md`](casos-de-uso/UC-CFG-05-entrar-no-servico.md) — **UC-CFG-05 — Entrar no serviço.** O Auditor sobe o serviço e abre o web no navegador.

## Demanda

- [`docs/casos-de-uso/UC-DEM-01-registrar-demanda.md`](casos-de-uso/UC-DEM-01-registrar-demanda.md) — **UC-DEM-01 — Registrar a demanda.** A IA externa entrega uma demanda pronta para o gate do Auditor. O entendimento e o desenho acontecem fora; o CLI só recebe o resultado.
- [`docs/casos-de-uso/UC-DEM-02-decidir-demanda.md`](casos-de-uso/UC-DEM-02-decidir-demanda.md) — **UC-DEM-02 — Decidir a demanda.** O Auditor aprova a demanda antes de qualquer planejamento, ou a recusa. Aprovada, nasce o trabalho de planejar. Recusada, a demanda se encerra.
- [`docs/casos-de-uso/UC-DEM-03-cancelar-demanda.md`](casos-de-uso/UC-DEM-03-cancelar-demanda.md) — **UC-DEM-03 — Cancelar a demanda.** O Auditor tira uma demanda do andamento. Ela não volta. Tickets, execuções, decisões, evidências e custos ficam.
- [`docs/casos-de-uso/UC-DEM-04-arquivar-demandas.md`](casos-de-uso/UC-DEM-04-arquivar-demandas.md) — **UC-DEM-04 — Arquivar demandas concluídas.** O Auditor arquiva, de uma vez, as demandas concluídas de um projeto que ainda não estão arquivadas.

## Planejamento

- [`docs/casos-de-uso/UC-PLAN-01-planejar-trabalho.md`](casos-de-uso/UC-PLAN-01-planejar-trabalho.md) — **UC-PLAN-01 — Planejar o trabalho.** O agente separa a demanda em rodadas e tickets. Cada rodada termina num pull request. O conjunto das rodadas resolve a demanda.
- [`docs/casos-de-uso/UC-PLAN-02-decidir-planejamento.md`](casos-de-uso/UC-PLAN-02-decidir-planejamento.md) — **UC-PLAN-02 — Decidir o planejamento.** O Auditor aprova a divisão em rodadas e tickets, ou a devolve ao mesmo agente com o motivo. Aprovada, a árvore congela e as rodadas podem começar.

## Execução

- [`docs/casos-de-uso/UC-EXE-01-disparar-trabalho.md`](casos-de-uso/UC-EXE-01-disparar-trabalho.md) — **UC-EXE-01 — Disparar o trabalho.** A Fabriquinha escolhe os tickets abertos e inicia cada um. O agente não escolhe o próximo trabalho.
- [`docs/casos-de-uso/UC-EXE-02-acompanhar-execucao.md`](casos-de-uso/UC-EXE-02-acompanhar-execucao.md) — **UC-EXE-02 — Acompanhar a execução.** O Auditor acompanha a sessão autônoma, responde o agente, interrompe ou devolve um ticket que falhou. O gate continua na página da demanda.
- [`docs/casos-de-uso/UC-EXE-03-conversar-sobre-sessao.md`](casos-de-uso/UC-EXE-03-conversar-sobre-sessao.md) — **UC-EXE-03 — Conversar sobre a sessão.** O Auditor abre uma consulta sobre a sessão do ticket. A consulta copia a conversa da sessão principal e só pergunta.

## Sessão

- [`docs/casos-de-uso/UC-SES-01-sessao-em-aberto.md`](casos-de-uso/UC-SES-01-sessao-em-aberto.md) — **UC-SES-01 — Sessão em aberto.** O Auditor e o agente combinam o ajuste do pull request reprovado. O agente abre tickets de implementação na mesma rodada. O código não muda aqui.

## Implementação

- [`docs/casos-de-uso/UC-IMPL-01-implementar-solucao.md`](casos-de-uso/UC-IMPL-01-implementar-solucao.md) — **UC-IMPL-01 — Implementar a solução.** O agente implementa um ticket de implementação na worktree da rodada. Testes e SonarQube aprovados, a Fabriquinha grava o commit.

## Revisão

- [`docs/casos-de-uso/UC-REV-01-revisar-rodada.md`](casos-de-uso/UC-REV-01-revisar-rodada.md) — **UC-REV-01 — Revisar a rodada.** O agente revisa o que a rodada implementou. Aprovado, a rodada segue para o encerramento. Reprovado, abre tickets de implementação na mesma rodada.
- [`docs/casos-de-uso/UC-REV-02-revisar-demanda.md`](casos-de-uso/UC-REV-02-revisar-demanda.md) — **UC-REV-02 — Revisar a demanda.** O agente revisa a demanda inteira em `fabriquinha/I-n` e faz a revisão adversarial. Reprovado, abre rodadas de correção.

## Encerramento

- [`docs/casos-de-uso/UC-ENC-01-encerrar-rodada.md`](casos-de-uso/UC-ENC-01-encerrar-rodada.md) — **UC-ENC-01 — Encerrar a rodada.** O agente documenta a rodada. A Fabriquinha monta o pull request e só leva a rodada ao gate quando ele abre sem conflito e sem check falho.
- [`docs/casos-de-uso/UC-ENC-02-decidir-pull-request.md`](casos-de-uso/UC-ENC-02-decidir-pull-request.md) — **UC-ENC-02 — Decidir o pull request da rodada.** O Auditor dá a nota de qualidade e aprova ou reprova o pull request contra `fabriquinha/I-n`. Aprovado, a Fabriquinha mergeia na branch da demanda. Reprovado, abre a sessão em aberto.
- [`docs/casos-de-uso/UC-ENC-03-encerrar-demanda.md`](casos-de-uso/UC-ENC-03-encerrar-demanda.md) — **UC-ENC-03 — Encerrar a demanda.** A Fabriquinha publica o pull request de `fabriquinha/I-n` contra a branch base, faz o merge e limpa as branches e as worktrees. O Auditor não participa deste merge.

## Consulta

- [`docs/casos-de-uso/UC-PRJ-01-consultar-projeto.md`](casos-de-uso/UC-PRJ-01-consultar-projeto.md) — **UC-PRJ-01 — Consultar o projeto.** O Auditor vê as demandas de um projeto, a fase de cada uma e quais estão fora do andamento.
- [`docs/casos-de-uso/UC-PRJ-02-ver-heatmap.md`](casos-de-uso/UC-PRJ-02-ver-heatmap.md) — **UC-PRJ-02 — Ver o heatmap.** O Auditor vê onde o código da branch base mais mudou nos últimos 12 meses.
- [`docs/casos-de-uso/UC-MET-01-ver-metricas.md`](casos-de-uso/UC-MET-01-ver-metricas.md) — **UC-MET-01 — Ver as métricas.** O Auditor lê um período de meses: o mês em foco contra a média dos meses anteriores, por etapa, por modelo e por demanda. A página só lê.
- [`docs/casos-de-uso/UC-HIS-01-ver-historico.md`](casos-de-uso/UC-HIS-01-ver-historico.md) — **UC-HIS-01 — Ver o histórico.** O Auditor filtra as sessões terminadas e lê a prévia. Abrir o chat, conversar sobre a sessão ou exportar o Markdown partem daqui.
- [`docs/casos-de-uso/UC-AUD-01-auditar-historico.md`](casos-de-uso/UC-AUD-01-auditar-historico.md) — **UC-AUD-01 — Auditar o histórico.** O Auditor pede a leitura das sessões terminadas de um mês, de todos os projetos, e conversa com um agente só de leitura.

## Atributos de qualidade

- [`docs/atributos-de-qualidade.md`](atributos-de-qualidade.md) — Métricas de manutenibilidade, paralelização, RAM e HD, com a justificativa e o valor de referência.
