# UC-DEM-03 — Cancelar a demanda

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página da demanda |

## Objetivo

O Auditor tira uma demanda do andamento. Ela não volta. Tickets, execuções, decisões, evidências e custos ficam.

## Pré-condições

A demanda não está nas fases `concluida`, `arquivada` nem `cancelada`. O botão fica na página dela. Na concluída e na arquivada, o botão aparece desabilitado, com a frase de que ela já terminou. Na cancelada, o botão some. O agente não cancela.

Na fase `aprovacao_demanda`, a recusa do gate continua em UC-DEM-02 e grava o motivo. Este botão também cancela, e não pede motivo.

## Fluxo principal

1. O Auditor abre **Cancelar demanda**. O diálogo diz que ela sai do andamento e não volta, e que nada do histórico é apagado.
2. A lista mostra os tickets abertos ou que falharam, que serão fechados; a execução em andamento, que será parada; e as demandas dependentes, com projeto e fase.
3. **Voltar** não muda nada. **Cancelar demanda** confirma.
4. A Fabriquinha confere de novo e, só então, para toda execução em andamento desta demanda e das dependentes. Um diálogo pendente recebe cancelado e o processo sai, como no abortar de UC-EXE-02.
5. Numa transação só, cancela esta demanda e as dependentes, diretas e indiretas, de qualquer projeto, que ainda não estejam nas fases `concluida`, `arquivada` nem `cancelada`. As demandas das quais esta depende não mudam.
6. Conta o uso da execução que parou. A falha dessa conta não desfaz o cancelamento.

## O que muda

A demanda passa à fase `cancelada`. A mudança de fase guarda a data e a fase de onde saiu. Cada ticket que muda de status guarda a mudança com a data.

| O que estava | Passa a |
|---|---|
| Ticket `aberto` ou `falhou` | `fechado`, sem o erro |
| Ticket `em_execucao` | `cancelado` |
| Execução em andamento | `falha`, com o erro "demanda cancelada pelo Auditor" |
| Ticket concluído | Continua concluído |

A sessão fica terminada. A demanda sai do andamento e o agendador não a pega (UC-EXE-01).

## Branches e worktrees

Quando a branch `fabriquinha/I-n` existir (UC-PLAN-02), a Fabriquinha fecha sem merge o pull request ainda aberto desta demanda e o de cada rodada. Remove as worktrees das rodadas e as temporárias. Apaga, no local e no `origin`, `fabriquinha/I-n` e `fabriquinha/I-n-R-*`. A branch base não se move. O mesmo vale para cada dependente cancelada junto.

Limpeza que deixa worktree, pasta, branch ou pull request aberto não desfaz o cancelamento. A página mostra o que sobrou, com o caminho e o comando.

## Pós-condições

A demanda e as dependentes estão canceladas, com o histórico na página. As branches e as worktrees delas não existem mais. A branch base segue sem esta demanda.
