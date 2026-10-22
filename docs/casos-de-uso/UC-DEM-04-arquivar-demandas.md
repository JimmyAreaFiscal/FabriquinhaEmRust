# UC-DEM-04 — Arquivar demandas concluídas

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página do projeto |

## Objetivo

O Auditor arquiva, de uma vez, as demandas concluídas de um projeto que ainda não estão arquivadas.

## Pré-condições

O projeto existe. O botão **Arquivar concluídas** fica na página dele. Sem demanda na fase `concluida`, o botão fica desabilitado. O agente não arquiva.

## Fluxo principal

1. O Auditor abre **Arquivar concluídas**. O diálogo diz quantas demandas concluídas daquele projeto serão arquivadas, que elas saem desta lista e que nada é apagado.
2. **Cancelar** não muda nada. **Arquivar** confirma.
3. Numa transação só, a Fabriquinha leva cada demanda deste projeto na fase `concluida` à fase `arquivada`. Cada mudança de fase guarda a data.
4. A página se redesenha. A falha fecha o diálogo e mostra o erro.

## Regras

| Demanda | Efeito |
|---|---|
| `concluida` deste projeto | Passa a `arquivada`, com a data da mudança. |
| `arquivada` | Não muda. A data do arquivamento de antes fica. |
| Outra fase | Não muda. |
| De outro projeto | Não muda. |

A cancelada não entra: cancelada e `concluida` são estados diferentes (UC-DEM-03).

Só muda a fase, de `concluida` para `arquivada`. Nada é apagado.

A arquivada sai da lista e continua acessível pela opção explícita de UC-PRJ-01. O agendador não a pega (UC-EXE-01).

## Pós-condições

As demandas concluídas daquele projeto que ainda não estavam arquivadas estão arquivadas. As demais demandas não mudam. Arquivar de novo não acha mais nenhuma.
