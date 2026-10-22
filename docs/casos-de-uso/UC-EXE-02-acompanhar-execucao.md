# UC-EXE-02 — Acompanhar a execução

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, chat da sessão |

## Objetivo

O Auditor acompanha a sessão autônoma, responde o agente, interrompe ou devolve um ticket que falhou. A decisão de gate continua na página da demanda. A sessão em aberto continua em UC-SES-01.

## Pré-condições

O ticket foi iniciado por UC-EXE-01. Todas as execuções dele seguem na mesma sessão, e o chat mostra as tentativas anteriores. Os eventos ao vivo chegam enquanto a execução está em andamento.

## Enquanto roda

1. O Auditor escreve uma mensagem. **Redirecionar** interrompe o passo atual. **Enfileirar** entra depois desse passo.
2. Antes de o processo subir, a Fabriquinha recusa a mensagem e pede para esperar a preparação da worktree.
3. Depois que o agente terminou e a Fabriquinha ainda confere as evidências, a mensagem espera o resultado.
4. **Abortar** cancela um diálogo pendente, manda o processo parar e leva o ticket a `falhou`, com o erro "abortada pelo Auditor". O que estiver por commitar fica na worktree. A rodada, ou a demanda quando o ticket não tem rodada, para até a reabertura (UC-EXE-01). Sem processo no ar, o botão diz que não há o que abortar.

O tempo limite é o de UC-CFG-01 e conta só o tempo ativo. Com um diálogo aberto, o relógio para. Ao estourar, a Fabriquinha manda `SIGTERM` ao grupo do processo, espera 10 segundos, manda `SIGKILL` e leva o ticket a `falhou`, com o erro "tempo limite de N min".

## Diálogo

O agente pergunta pela ferramenta `perguntar`, ou por um diálogo de extensão do Auditor. A sessão espera. O formulário é escolha, confirmação, texto ou área de texto. Na escolha da `perguntar`, o último item é **Outros**, com texto obrigatório, e a múltipla escolha usa caixas. **Cancelar** responde cancelado e o agente segue. **Responder** envia o que foi preenchido. Um diálogo que o processo limita no tempo se encerra sozinho nesse prazo.

## Fluxo alternativo — falha

O chat mostra o erro. O Auditor pode trocar o modelo do ticket, como no seletor de UC-PLAN-02. Vazio volta ao padrão da etapa. Modelo que o pi não lista é recusado e o ticket continua `falhou`. Se a troca do modelo passar e a reabertura não, o modelo novo permanece.

**Responder e retomar** devolve o ticket a `aberto`. A instrução é opcional e cabe em 1.000 palavras. A execução seguinte, pelo agendador ou por Executar agora, continua a mesma sessão, e o prompt começa por essa instrução. A Fabriquinha a consome nesse início.

## Serviço reiniciado

Na subida, a execução autônoma que ficou em andamento recebe `SIGKILL` no grupo e o ticket vai a `falhou`, com o erro "serviço reiniciado". A sessão interativa continua `em_execucao`; a mensagem seguinte a retoma (UC-SES-01, UC-CFG-01).

## Histórico na mensagem

Neste chat e na reabertura, o Auditor anexa o histórico de outras sessões. A auditoria do mês não anexa (UC-AUD-01). A Fabriquinha grava cada uma em Markdown em `$FABRIQUINHA_SAIDA` e acrescenta à mensagem uma linha por anexo, com o caminho, para o agente ler antes de responder. O arquivo traz o cabeçalho da sessão, as mensagens na íntegra, cada chamada de ferramenta com nome e argumentos, e o resultado cortado em 2.000 caracteres. A mesma anexação vale na reabertura.

## Pós-condições

A sessão segue em andamento, ou o ticket está `falhou` e segura a fila, ou voltou a `aberto` com a instrução gravada para a próxima execução. Sessão terminada fica só para leitura (UC-HIS-01).
