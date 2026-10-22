# UC-EXE-01 — Disparar o trabalho

| | |
|---|---|
| Ator | Fabriquinha; o Auditor no botão Executar agora |
| Superfície | Agendador e a página da demanda |

## Objetivo

A Fabriquinha escolhe os tickets abertos e inicia cada um. O agente não escolhe o próximo trabalho.

## Pré-condições

O serviço está no ar. O intervalo, o tempo limite e o interruptor são os de UC-CFG-01. Com o agendador desligado, só o Executar agora dispara.

## Fluxo principal

1. A cada intervalo, com o agendador ligado, a Fabriquinha busca os tickets elegíveis.
2. Inicia cada um. A recusa de um fica no log e não impede os outros.
3. Na fase `execucao`, inicia no máximo um ticket por rodada. Rodadas sem dependência pendente correm juntas, cada uma na sua worktree (UC-IMPL-01).
4. Nas outras fases, inicia no máximo um ticket da demanda. O planejamento, a revisão da demanda e o encerramento da demanda são esses tickets.
5. O encerramento da demanda não abre agente (UC-ENC-03). As outras etapas abrem a sessão do caso delas.

A ordem, em cada rodada e em cada demanda, é a que UC-IMPL-01 já fixa para a rodada: o ticket aberto mais antigo cujas dependências estão concluídas.

## Ticket elegível

Entra o ticket que está `aberto` e na fase em que a demanda, e a rodada quando ele tem rodada, já está.

| Trava | Efeito |
|---|---|
| Dependência de demanda | O planejamento espera todas as demandas de que esta depende concluídas (UC-DEM-02). |
| Dependência de rodada | A rodada espera as rodadas de que depende concluídas (UC-PLAN-02). |
| Dependência de ticket | O ticket espera os tickets de que depende concluídos. |
| Execução em andamento | A rodada, ou a demanda quando o ticket não tem rodada, não recebe outra. |
| Ticket que falhou | A mesma rodada, ou a mesma demanda quando o ticket não tem rodada, para. O trabalho sem commit fica na worktree. O Auditor reabre com uma instrução; a execução seguinte retoma a mesma sessão. |
| Demanda cancelada | Não entra (UC-DEM-02). |
| Demanda arquivada | Não entra (UC-DEM-04). |
| Conflito de merge | Enquanto o ticket "Resolver o conflito de merge" está aberto ou falhou, o ticket de encerramento da demanda não entra. Quando o de conflito conclui, o encerramento volta a entrar (UC-ENC-03). |

Sessão em aberto e gates não entram neste ciclo. A sessão em aberto abre na hora da reprovação do pull request (UC-SES-01).

## Executar agora

O botão dispara o ticket que o ciclo dispararia agora nessa rodada ou nessa demanda, sem esperar o intervalo. Outro ticket aberto, ou uma execução já em andamento nessa rodada ou nessa demanda, deixa o botão desabilitado com o motivo.

## No mesmo ciclo

Para cada projeto sem varredura da branch base há 24 horas, a Fabriquinha busca `origin/<branch base>` e conta, nos commits novos que não são de ticket nem merge, as linhas de ticket que eles reescreveram. Isso alimenta a métrica de alterações posteriores (UC-IMPL-01). Falha de um projeto só vai ao log. O ciclo não espera essa varredura.

## Pós-condições

Cada ticket elegível está em execução, no caso da etapa dele. O que não passou numa trava continua aberto.
