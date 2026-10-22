# UC-CFG-01 — Configurar o agendador

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Configuração → Agendador |

## Objetivo

O Auditor decide quando a Fabriquinha procura trabalho sozinha e por quanto tempo uma execução pode durar.

## Pré-condições

Existe uma única configuração do serviço. Na primeira subida o agendador está desligado, o intervalo é 5 minutos, o tempo limite da execução é 120 minutos e a sessão ociosa para após 30 minutos.

## Fluxo principal

1. O Auditor abre Configuração → Agendador.
2. A tela mostra o interruptor, os três tempos em minutos, a próxima busca e as execuções autônomas em andamento.
3. A próxima busca aparece somente com o agendador ligado.
4. Cada execução em andamento aparece numa linha: execução, demanda, ticket e há quanto tempo roda.
5. O Auditor ajusta o interruptor, o intervalo, o tempo limite e a ociosidade, e salva.
6. A Fabriquinha grava a configuração inteira e mostra o estado atualizado.

## Fluxos alternativos

- **Campo inválido.** A Fabriquinha recusa a gravação e mostra o erro ao lado do campo. A configuração anterior permanece.
- **Campo ausente.** O valor volta ao padrão desta configuração.

## Regras

| Campo | Regra |
|---|---|
| Agendador ativo | Desligado, a Fabriquinha não dispara trabalho sozinha. O Auditor continua podendo pedir a execução de um ticket na página da demanda. |
| Intervalo | Inteiro de 1 a 1440 minutos. De quanto em quanto tempo o agendador procura trabalho. |
| Tempo limite | Inteiro de 10 a 1440 minutos. Vale para uma execução autônoma e conta só o tempo ativo. Serve para calibrar modelo local lento. |
| Ociosidade | Inteiro de 5 a 1440 minutos. Depois desse tempo sem mensagem, o processo de uma sessão interativa, de uma consulta (UC-EXE-03) ou da auditoria (UC-AUD-01) para. A próxima mensagem retoma a mesma sessão. |

## Pós-condições

A configuração gravada passa a valer para a próxima busca do agendador e para as execuções iniciadas depois do salvamento.
