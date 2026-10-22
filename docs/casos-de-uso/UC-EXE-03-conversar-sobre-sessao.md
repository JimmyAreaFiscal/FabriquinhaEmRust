# UC-EXE-03 — Conversar sobre a sessão

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, chat da sessão |

## Objetivo

O Auditor abre uma consulta sobre a sessão do ticket. A consulta copia a conversa da sessão principal e só pergunta.

## Pré-condições

O ticket já tem sessão principal, com a conversa gravada. A execução desse ticket, se houver, está ociosa: o agente não trabalha e não há diálogo aberto. A demanda concluída também aceita a consulta.

## Fluxo principal

1. No chat terminado, o Auditor clica em **Conversar sobre esta sessão**.
2. O mesmo pedido está na barra do chat e na prévia do histórico, com o rótulo **Conversar sobre**.
3. Se o processo do ticket está ocioso, a Fabriquinha o para e espera o término da execução.
4. A Fabriquinha abre uma sessão nova com a cópia da sessão principal do ticket.
5. O processo sobe à espera da primeira pergunta.
6. Até a cópia existir, o chat mostra a conversa da principal.
7. O Auditor escreve a pergunta.
8. A mensagem fica na consulta.
9. Sem mensagem pelo tempo de UC-CFG-01, o processo para.
10. A mensagem seguinte retoma a mesma consulta.
11. A retomada usa a cópia já gravada.
12. Se a primeira execução parou antes de gravar a cópia, a retomada copia.
13. Anexar o histórico de outra sessão segue UC-EXE-02.

## Regras

| Regra | Efeito |
|---|---|
| Ferramentas | A consulta sai sem `edit`, `write`, `bash` e `powershell`. O diálogo da extensão segue UC-EXE-02. |
| Ticket e demanda | O status do ticket e a fase da demanda permanecem. O agente da consulta só pergunta. |
| Vaga da demanda | A consulta não ocupa a rodada nem a demanda. O agendador segue UC-EXE-01. |
| Custo | O custo entra na demanda, separado das etapas, e soma só o uso desta execução. O histórico copiado fica de fora. |
| Ticket concluído | Com o processo parado e o ticket concluído, fora do gate do pull request (UC-ENC-02), a consulta fica terminada. Outra pergunta abre outra consulta. |
| Reinício | A execução em andamento termina com o erro "serviço reiniciado". O ticket permanece. Em conversa, a mensagem seguinte retoma a mesma consulta. |
| Sem sessão | Sem sessão principal, a Fabriquinha recusa. Sem a conversa gravada, pede para esperar o agente responder. |
| Agente ocupado | Com o agente trabalhando ou com diálogo aberto, a Fabriquinha recusa e pede para esperar. |
| Gate e falha | Continuam na sessão principal (UC-EXE-02). |

## Pós-condições

A consulta está na sessão nova, com a conversa copiada. O ticket e a fase da demanda estão como antes do clique. A sessão principal segue sem as mensagens da consulta.
