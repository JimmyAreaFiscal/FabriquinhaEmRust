# UC-AUD-01 — Auditar o histórico

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Histórico e o chat da auditoria |

## Objetivo

O Auditor pede a leitura das sessões terminadas de um mês, de todos os projetos. Um agente só de leitura conversa com ele sobre essa leitura. Esta sessão fica fora de demanda, de ticket e de execução de trabalho. A lista das sessões continua em UC-HIS-01.

## Pré-condições

O Auditor está no Histórico. O mês do pedido é `AAAA-MM`. A ociosidade do pi é a de UC-CFG-01.

## Fluxo principal

1. O Auditor escolhe o mês e pede **Auditar**. O seletor oferece os 12 meses até o corrente, no fuso do navegador, com o corrente escolhido. O mês fica fora dos filtros da lista.
2. A Fabriquinha toma as sessões terminadas de UC-HIS-01, de todos os projetos, cujo `desde` cai no mês, da mais antiga para a mais recente. O corte do mês é o do fuso do serviço, do início inclusive ao fim exclusive.
3. Sem sessão nesse mês, a Fabriquinha recusa com "Nenhuma sessão terminada em {mês por extenso} para auditar." O Auditor permanece no Histórico.
4. Com sessões, ela grava a pasta `auditorias/<sessão>/` e sobe o pi. O chat abre em `/audits/<sessão>` quando o agente inicia.
5. O chat mostra "Auditoria de {mês por extenso} iniciada" no lugar do prompt. A barra traz **Auditoria**, o mês, "todos os projetos", a quantidade de sessões, o estado, o modelo e o link **índice do mês**, noutra aba.
6. A coluna registra o mês, quantas sessões terminadas o agente recebeu, a contagem por projeto e que ele só propõe melhorias.
7. O agente lê o índice e responde. O Auditor pergunta na mesma conversa. O campo diz "Pergunte sobre a auditoria".
8. **Abortar** manda o pi parar. A pasta e a conversa ficam. O estado volta a conversa quando o pi sai.

Se o pi não chega a subir, ou a preparação falha, a Fabriquinha apaga a pasta e o JSONL da sessão. O erro começa por "o agente da auditoria não subiu". O Auditor permanece no Histórico.

## Regras

O pi sobe em modo `consulta`, com a pasta da auditoria como diretório de trabalho. Há no máximo um pi por sessão.

| Limite do agente | Regra |
|---|---|
| Ferramentas | O pi sobe sem `edit`, `write`, `bash` e `powershell`. |
| Subagente | Um subagente diferente do `scout` é recusado, com "A auditoria só usa subagentes de leitura (scout)." |
| Etapa | O pi sobe sem skill de etapa e sem modelo de etapa. |
| Ambiente | O ambiente marca a auditoria. As credenciais do serviço ficam de fora. `gh` e `az` leem o diretório vazio da pasta. |

O prompt manda ler o `indice.md`, chamar `subagent` uma vez, em paralelo, com três tarefas do `scout`, e responder nesta ordem: Oportunidades de melhoria, Custos, Qualidade, Melhorias propostas. Cada melhoria traz o problema e a evidência, a sessão ou o número do índice. As melhorias ficam no texto da resposta. Se `subagent` não existir, o agente diz isso e faz as três análises em sequência. Na pergunta seguinte, ele abre a sessão pedida.

| Seção | O que o agente olha |
|---|---|
| Oportunidades de melhoria | O que se repete e atrasa o trabalho nas conversas. |
| Custos | A seção Custos do índice e as sessões mais caras. |
| Qualidade | A seção Qualidade e as sessões das demandas com mais retrabalho ou correções. |

O `indice.md` traz o intervalo, a quantidade de sessões e cada projeto com a sua quantidade. Cada sessão terminada vira um Markdown em `sessoes/`, no formato do anexo de UC-EXE-02.

| Coluna da sessão | Valor |
|---|---|
| Terminada | O `desde` da sessão. |
| Projeto | O projeto da sessão. |
| Demanda | `I-n`. |
| Título | O título da demanda, ou "(sem título)". |
| Ticket | O `T-n` da sessão. |
| Etapa | A etapa gravada. |
| Modo | `autonomo`, `interativo` ou `consulta`. |
| Resultado | `concluida`, `falha`, `abortada` ou `cancelada`. |
| Custo | O custo da sessão, em `US$` com duas casas. |
| Arquivo | O caminho em `sessoes/`. |

Os custos do mês saem das métricas desse mês: o total em `US$` com duas casas e o número de chamadas, a tabela por modelo (provedor, modelo, chamadas, custo) e a tabela por etapa (execuções, falhas, reprovações, duração em segundos, custo). A tabela por etapa inclui planejar, implementar, revisar a rodada, encerrar a rodada, sessão em aberto e revisar a demanda, mesmo com zero. Execuções e falhas contam as autônomas e as interativas. As consultas entram no total e na tabela por modelo.

A qualidade lista cada demanda com sessão no mês, da menor `I-n` para a maior, e uma linha de total. Os números são os da demanda inteira, os que os casos já medem.

| Coluna da qualidade | Número |
|---|---|
| Linhas alteradas depois | Alterações posteriores (UC-IMPL-01). |
| Linhas alteradas por humanos | A parte dessa métrica que veio de commit humano na branch base. |
| Exigências da revisão | UC-REV-01 e UC-REV-02. |
| Bugs em arquivos revisados | UC-REV-01 e UC-REV-02. |
| Nota do pull request | A nota de UC-ENC-02. Sem nota, a célula fica vazia. |

A barra usa "rodando" enquanto o pi trabalha, sobe ou para, e "aguarda você · conversa" quando ele espera. O modelo é o da última resposta, ou "modelo padrão do pi" enquanto ele não respondeu.

A mensagem leva texto. Vazio, depois do trim, a rota recusa. Se o envio falha, o texto volta ao campo. Este envio leva só o texto.

Com o agente trabalhando, o campo sai e **Abortar** entra. A página avisa que a pergunta fica para quando ele terminar. A mensagem nesse estado é recusada, com "o agente da auditoria está trabalhando". Enquanto o pi ainda para, a mensagem é recusada, com "o agente da auditoria ainda está parando".

Com o pi vivo e ocioso, a pergunta vai a ele. Com o pi parado, a pergunta sobe outro pi na mesma sessão e na mesma pasta. A ociosidade é a de UC-CFG-01: com o agente assentado e sem diálogo pendente, esse prazo para o pi. A conversa continua à espera.

Na subida, a Fabriquinha manda `SIGKILL` ao grupo de cada pi de auditoria que ficou no ar; a pasta e a conversa ficam, e a mensagem seguinte sobe outro pi na mesma sessão.

## Pós-condições

A sessão de auditoria está aberta, com a pasta, o índice e uma conversa por sessão terminada do mês. O pi está no ar ou parado à espera da próxima pergunta. Se o mês não tem sessão terminada, ou o pi não sobe, não fica pasta nem sessão.
