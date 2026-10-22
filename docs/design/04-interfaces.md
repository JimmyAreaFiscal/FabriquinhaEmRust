# Design — Interfaces

Este documento fixa as fronteiras da Fabriquinha: quem produz cada interface, quem a consome e quanto a resposta pode levar.
Os pacotes estão em `01-pacotes.md`, as entidades em `02-dominio.md` e os serviços em `03-servicos.md`.

## Decisões

| # | Decisão | Motivo |
|---|---|---|
| I1 | O web e o CLI usam a mesma API em `/api`, com JSON. | Uma regra, uma rota (P6). |
| I2 | Leitura por recurso; ação por verbo do caso, em `POST`: `/api/demandas/I-12/aprovar`. | A rota lê como o caso de uso. |
| I3 | Sem autenticação (P9). O CLI do agente manda `Fabriquinha-Execucao: <id>`; o da IA externa manda `Fabriquinha-Sessao: <ferramenta>:<id>`. | O cabeçalho é escopo de regra ("só quem criou edita"), não barreira: o agente roda como o mesmo usuário do sistema. |
| I4 | O CLI do agente lê `GET /api/execucoes/{id}` e monta o caminho completo de `R-n` e `T-n`. | As rotas continuam uma só para o web e para o CLI. |
| I5 | O anexo vai por caminho. O serviço confere que o arquivo está na worktree da execução, em `$FABRIQUINHA_SAIDA` ou no diretório em que o comando rodou, e copia em blocos de 1 MB. | O arquivo é copiado uma vez, sem passar inteiro pelo pedido HTTP, até 500 MiB. |
| I6 | Toda falha responde `{ "erro": "…", "campos": { "<campo>": "…" } }`. 400 é campo inválido, 404 é recurso ausente, 409 é regra ou fase que não permite, 403 é `Host` ou `Origin` recusado (UC-CFG-05). | O web mostra o erro ao lado do campo; o CLI o escreve no stderr. |
| I7 | Ação longa responde quando a decisão está gravada e segue em segundo plano. O web acompanha pela releitura ou pelo SSE. | O pedido HTTP não espera agente, push nem conferência de PR. |

## Valores de referência

O p99 conta do pedido recebido à resposta enviada, só no processo da Fabriquinha.
Quando a resposta depende de rede ou de ferramenta externa (git remoto, `gh`, `az`, SonarQube, pi), a coluna diz **teto**: o limite em que a Fabriquinha desiste e responde com erro.

A coluna Origem diz de onde sai o valor:

- **atributos**: já está em `atributos-de-qualidade.md` ou num caso de uso.
- **proposto**: novo neste documento, a validar e levar aos atributos de qualidade.

## 1. API HTTP

Produtor: módulo `web` do serviço.

| Interface | Consumidores | Justificativa | p99 | Origem |
|---|---|---|---|---|
| `GET /api/agendador` | Angular | O Auditor decide quando a Fabriquinha procura trabalho e quanto tempo uma execução pode durar. | 50 ms | proposto |
| `PUT /api/agendador` | Angular | Idem. | 100 ms | proposto |
| `GET /api/projetos`, `GET /api/projetos/{p}` | Angular; CLI do agente (`projeto ver`) | O Auditor mantém os repositórios em que a Fabriquinha trabalha, e o agente lê a configuração do projeto. | 200 ms, com a lista de demandas | atributos |
| `POST /api/projetos`, `PUT /api/projetos/{p}` | Angular | Idem. A conferência roda `git`, `gh` ou `az` e lista os modelos do pi. | teto 15 s | proposto |
| `DELETE /api/projetos/{p}` | Angular | Idem. Apaga o banco e as pastas do projeto. | banco 1 s; disco teto 60 s | proposto |
| `GET /api/projetos/{p}/heatmap` | Angular | O Auditor vê onde o código da branch base mais mudou nos últimos 12 meses. | 3 s, até 128 MB de RAM | atributos |
| `POST /api/projetos/{p}/arquivar` | Angular | O Auditor tira da lista, de uma vez, as demandas concluídas de um projeto. | 200 ms | proposto |
| `GET /api/modelos` | Angular | O Auditor escolhe o modelo de cada etapa ou ticket entre os que o pi oferece. | teto 5 s (sobe um pi para listar) | proposto |
| `GET /api/inicio` | Angular | O Auditor vê o que espera a decisão dele e o que está rodando. | 200 ms, até 30 demandas em andamento | atributos |
| `POST /api/demandas`, `PATCH /api/demandas/{I}`, `POST /api/demandas/{I}/enviar` | CLI da IA externa | A IA externa entrega uma demanda pronta para o gate. | 200 ms | proposto |
| `GET /api/demandas/{I}` | CLI da IA externa e do agente; Angular | O Auditor e o agente leem a demanda. | 200 ms | proposto |
| `POST /api/demandas/{I}/aprovar`, `recusar`; `PUT .../dependencias` | Angular | O Auditor aprova a demanda antes do planejamento ou a recusa. | 200 ms | proposto |
| `POST /api/demandas/{I}/cancelar` | Angular | O Auditor tira a demanda do andamento. A resposta espera o processo parar (`SIGTERM`, 10 s, `SIGKILL`); a limpeza de branches e worktrees segue depois (I7). | 12 s | proposto |
| `GET /api/schemas/{tipo}` | CLI da IA externa e do agente | Quem executa a etapa grava a prova do que fez, no formato que a Fabriquinha confere. | 50 ms | proposto |
| `PUT .../tickets/{T}/evidencias/{tipo}` | CLI da IA externa e do agente | Idem. Sem anexo, ou com anexo até 20 MiB. | 300 ms | proposto |
| `PUT .../tickets/{T}/evidencias/{tipo}`, com vídeo | CLI do agente | Idem. Cópia em blocos de 1 MB, até 500 MiB. | 5 s | proposto |
| `GET .../tickets/{T}/evidencias/{tipo}` | CLI do agente; Angular | O agente e o Auditor leem uma evidência já gravada. | 200 ms | proposto |
| `.../rodadas` e `.../tickets`: `POST`, `PATCH`, `DELETE` | CLI do agente; Angular nos ajustes do gate | O agente divide o trabalho em rodadas e tickets, e o Auditor ajusta a divisão antes de aprovar. Inclui regerar a `arvore_planejamento`. | 300 ms | proposto |
| `POST .../planejamento/aprovar` | Angular | O Auditor aprova a divisão do trabalho. Publica `fabriquinha/I-n` antes de gravar. | teto 60 s (push) | proposto |
| `POST .../planejamento/reprovar` | Angular | O Auditor devolve a divisão ao agente com o motivo. | 200 ms | proposto |
| `POST .../rodadas/{R}/pr/aprovar` | Angular | O Auditor dá a nota e aprova o pull request da rodada. Mergeia antes de gravar. | teto 60 s (`gh`, `az`) | proposto |
| `POST .../rodadas/{R}/pr/reprovar` | Angular | O Auditor reprova o pull request; a sessão em aberto sobe em segundo plano (I7). | 200 ms | proposto |
| `POST .../tickets/{T}/revisao` | CLI do agente | O agente aprova o que a rodada ou a demanda entregou, ou exige correções. | 200 ms | proposto |
| `POST .../tickets/{T}/executar` | Angular | O Auditor dispara um ticket sem esperar o agendador. A execução segue em segundo plano. | 50 ms | atributos |
| `POST .../tickets/{T}/retomar`; `PUT .../tickets/{T}/modelo` | Angular | O Auditor devolve à fila um ticket que falhou, com instrução ou outro modelo. | 200 ms | proposto |
| `POST .../tickets/{T}/consulta` | Angular | O Auditor faz perguntas sobre uma sessão sem mudar o ticket. Para o processo ocioso do ticket antes de copiar. | 12 s | proposto |
| `GET /api/execucoes/{id}` | CLI do agente | O agente descobre a demanda da execução em curso para citar rodadas e tickets só pelo número. | 50 ms | proposto |
| `GET /api/sessoes/{id}` | Angular | O Auditor abre o chat de uma sessão. Sessão até 32 MB. | 1 s | proposto |
| `POST /api/sessoes/{id}/mensagens`, `dialogo` | Angular | O Auditor conversa com o agente e responde às perguntas dele. | 100 ms | proposto |
| `POST /api/sessoes/{id}/abortar` | Angular | O Auditor interrompe o agente. A resposta espera o processo sair. | 12 s | proposto |
| `POST /api/sessoes/{id}/concluir` | Angular | O Auditor conclui a sessão em aberto e a rodada volta ao desenvolvimento. Descarta o que estava por commitar. | 2 s, mais a espera na fila git | proposto |
| `GET /api/sessoes/{id}/eventos` (SSE) | Angular | O Auditor acompanha, ao vivo, o que o agente faz. Do evento do pi ao envio no SSE. | 100 ms | atributos |
| `GET /api/sessoes/{id}/markdown` | Angular | O Auditor baixa a conversa de uma sessão em Markdown. | 2 s | proposto |
| `GET /api/historico` | Angular | O Auditor filtra as sessões terminadas e lê a prévia de cada uma. | 400 ms | atributos |
| `POST /api/auditorias` | Angular | O Auditor conversa com um agente só de leitura sobre as sessões de um mês. Gera a pasta do mês e sobe o pi. | teto 60 s | proposto |
| `GET /api/metricas` | Angular | O Auditor compara um mês com a média dos anteriores, por etapa, modelo e demanda. | 300 ms, 12 meses | atributos |
| `/static/*` e `index.html` | Navegador | O navegador carrega o web embutido no binário. | 50 ms | proposto |

### Rotas e casos

| Prefixo | Casos |
|---|---|
| `/api/agendador` | UC-CFG-01 |
| `/api/projetos` | UC-CFG-02, UC-CFG-03, UC-CFG-04, UC-PRJ-01, UC-PRJ-02, UC-DEM-04 |
| `/api/modelos` | UC-CFG-02, UC-PLAN-02, UC-EXE-02 |
| `/api/inicio` | UC-INI-01 |
| `/api/demandas` | UC-DEM-01 a 03, UC-PLAN-01, UC-PLAN-02, UC-EXE-01, UC-EXE-02, UC-EXE-03, UC-SES-01, UC-REV-01, UC-REV-02, UC-ENC-01, UC-ENC-02 |
| `/api/schemas` | UC-DEM-01, UC-REV-01, UC-REV-02, UC-ENC-01 |
| `/api/execucoes` | todos os casos com agente |
| `/api/sessoes` | UC-EXE-02, UC-EXE-03, UC-SES-01, UC-HIS-01, UC-AUD-01 |
| `/api/historico` | UC-HIS-01 |
| `/api/auditorias` | UC-AUD-01 |
| `/api/metricas` | UC-MET-01 |

### Contexto da execução para o CLI

`GET /api/execucoes/{id}` devolve os identificadores necessários para o agente montar os caminhos completos:

```json
{"execucao":"12","projeto":"calculadora","demanda":"I-3","ticket":"T-2","rodada":"R-1"}
```

`rodada` é `null` para tickets da demanda. A leitura atende tanto execuções de uma sessão quanto execuções diretamente vinculadas ao ticket. ID inválido responde 400, execução ausente responde 404 e execução sem contexto de demanda/ticket responde 409, sempre no formato I6.

O CLI lê o endereço em `<raiz>/web.url` e envia `Fabriquinha-Execucao` quando `FABRIQUINHA_EXECUCAO` está definido. `fabriquinha projeto ver` usa o projeto da execução; `fabriquinha demanda ver` usa sua demanda. Também é possível informar explicitamente `projeto ver <nome>` ou `demanda ver I-n`.

## 2. Processos e ferramentas

| Interface | Produtor | Consumidores | Justificativa | p99 | Origem |
|---|---|---|---|---|---|
| `fabriquinha servidor` | `cli` | Auditor, systemd | O Auditor sobe o serviço e abre o web no navegador. Da subida ao `{"porta":…}` no stdout, com o banco no ar. | 5 s | proposto |
| `fabriquinha abrir` | `cli` | Auditor | Idem. | 200 ms, sem o navegador | proposto |
| `fabriquinha demanda …` | `cli` | IA externa (pi, Claude Code, Codex) | A IA externa registra a demanda e as evidências sem abrir o banco. | o p99 da rota, mais 50 ms | proposto |
| `fabriquinha projeto`, `demanda ver`, `rodada`, `ticket`, `evidencia`, `revisao` | `cli` | Agente, pelo bash do pi | O agente lê o contexto e grava o resultado da etapa pelo serviço. | o p99 da rota, mais 50 ms | proposto |
| Argumentos de subida do pi (`--skill`, `-e`, `--session-dir`, modelo) | GerenteDeAgentes | pi | Cada etapa sobe o agente com as skills, as tools e o modelo dela. Do pedido de subida ao processo no ar, sem a worktree e sem a preparação. | 5 s | proposto |
| Checkout da worktree | módulo `git` | `executar_etapa` | A rodada trabalha na própria worktree. Até 500 MB, sem `target/` e sem `node_modules`. | 10 s | atributos |
| Preparação da worktree | módulo `comandos` | `executar_etapa` | Os comandos do projeto instalam as dependências antes do agente. | teto 30 min | atributos |
| RPC JSONL: `prompt`, `steer`, `follow_up`, `abort` | pi | GerenteDeAgentes | A Fabriquinha manda mensagens, redireciona e interrompe o agente. Do pedido HTTP à escrita no stdin do pi. | 100 ms | proposto |
| RPC JSONL: `get_available_models` | pi | GerenteDeAgentes | A Fabriquinha lista os modelos que o pi oferece. | teto 5 s | proposto |
| Diálogo RPC: `select`, `confirm`, `input`, `editor` | pi (`perguntar`) | GerenteDeAgentes, que repassa ao Angular | O agente pergunta ao Auditor e espera a resposta. Do pedido do pi ao SSE. | 100 ms | atributos |
| Tempo do agente numa execução | pi | GerenteDeAgentes | O agente trabalha até terminar a etapa. Conta só o tempo ativo. | teto do agendador, padrão 120 min, máximo 1440 | atributos |
| Tool `guarda` | `pi-fabriquinha` | Agente | O agente é impedido de criar, alterar ou mergear pull request. Decisão sobre um comando do bash. | 10 ms | proposto |
| Tool `perguntar` | `pi-fabriquinha` | Agente | O agente pergunta ao Auditor e espera a resposta. A espera é do Auditor e para o relógio. | — | — |
| Tool `navegador` | `pi-fabriquinha` | Agente | O agente usa um navegador headless para verificar e gravar a tela. Cada ação: abrir, clicar, digitar, ler, print. | teto 30 s por ação | proposto |
| `git` local: commit, worktree, branch, merge | sistema | módulo `git` | A Fabriquinha cria worktrees, commita com os hooks do repositório e mergeia. Espera na fila de um repositório ocupado por commit. | 100 ms de espera | atributos |
| `git` remoto: fetch, push | sistema | módulo `git` | Idem, contra o `origin`. | teto 30 min | atributos |
| `gh` e `az`: criar, ler, mergear e fechar pull request | sistema | módulo `hospedagem` | A Fabriquinha publica, confere e mergeia pull requests no GitHub ou no Azure DevOps. Cada chamada. | teto 60 s | proposto |
| Conferência de conflito e checks | módulo `hospedagem` | `publicar_pr` | A Fabriquinha só leva o PR ao gate sem conflito e sem check falho. Lê a cada 30 s. | teto 60 min | atributos |
| `sonar-scanner` | SonarQube | módulo `sonar` | A Fabriquinha só commita quando não há issue nos arquivos alterados. | teto 30 min | atributos |
| API web do SonarQube (`api/issues/search`) | SonarQube | módulo `sonar` | Idem: lê as issues da análise. | teto 30 s | proposto |
| Comandos de teste e fitness | projeto do Auditor | módulo `comandos` | A Fabriquinha confere testes e fitness functions antes do commit. Cada comando. | teto 30 min | atributos |

## 3. Dados e ambiente

| Interface | Produtor | Consumidores | Justificativa | p99 | Origem |
|---|---|---|---|---|---|
| PostgreSQL, por `sqlx`: leitura por chave | Docker | módulo `banco` | A Fabriquinha guarda o estado do processo num banco que sobrevive ao reinício. | 10 ms | proposto |
| PostgreSQL: transação de um caso | Docker | módulo `banco` | Idem: a decisão de um caso grava junta. | 50 ms | proposto |
| Arquivos em `<raiz>`: cópia de anexo | módulo `arquivos` | serviço | Anexos e logs grandes ficam fora do banco. Blocos de 1 MB. | 5 s para 500 MiB | proposto |
| Arquivo JSONL da sessão: fim da conversa | pi | Histórico | A conversa do agente fica para leitura e auditoria. A prévia lê só o fim. | dentro dos 400 ms do Histórico | atributos |
| Arquivo JSONL da sessão: cópia para a consulta | pi | `conversar_sobre_sessao` | O Auditor faz novas perguntas sobre a mesma conversa. Até 32 MB. | 1 s | proposto |
| `<raiz>/templates` | Inicialização | pi | O pi recebe por caminho os arquivos do template embutido. Extração na subida. | 1 s | proposto |
| `precos.json` embutido | repositório | `custo.rs` | A Fabriquinha calcula o custo de cada uso pelo preço do modelo. Lido uma vez na subida. | — | — |
| `web.url` | serviço | `fabriquinha abrir` | O comando `abrir` descobre o endereço do serviço no ar. | — | — |
| `FABRIQUINHA_ROOT`, `_PORTA`, `_BANCO`, `_PI`, `SONAR_HOST_URL`, `SONAR_TOKEN` | Auditor (systemd) | serviço | O Auditor configura onde ficam os dados, o banco, o pi e o SonarQube. | — | — |
| `FABRIQUINHA_EXECUCAO`, `FABRIQUINHA_SAIDA` | GerenteDeAgentes | CLI e tools dentro do pi | O agente sabe em qual execução está e onde deixar os anexos. | — | — |

O travessão marca o que não tem resposta a medir: um valor lido uma vez ou uma espera que é do Auditor.

## Pendências

1. Os valores **propostos** ainda não estão em `atributos-de-qualidade.md`. Validados, recomendo levá-los para lá.
