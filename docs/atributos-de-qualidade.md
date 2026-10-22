# Atributos de qualidade

A Fabriquinha atende um Auditor, num processo só, em `127.0.0.1` (UC-CFG-05). O p99 da tabela é o trabalho desse processo. O tempo do agente continua o tempo limite do agendador (UC-CFG-01): padrão 120 minutos, teto 1440.

Os valores de disco saem de `~/fabriquinha` em 21/10/2026: 85 demandas, 939 execuções, 640 sessões, 21 GB. A latência é orçamento, para caber no relógio que já existe. O tick do agendador conta de minuto em minuto. Início e Projeto releem a cada 5 segundos (UC-INI-01, UC-PRJ-01).

## Manutenibilidade

O caso de uso é a regra. O módulo que decide cita o UC.

O núcleo não faz IO. Elegibilidade, gate, fase e cancelamento se testam sem git, sem agente e sem HTTP. Essa suíte termina em menos de 30 segundos.

O código usa os nomes do caso: demanda, rodada, ticket, evidência, gate.

Há uma instância. A porta é a trava. A fila git em memória vale por isso.

Etapa nova entra no enum, no schema da evidência, no coletor e no caso de uso. O diff não muda a elegibilidade das outras etapas.

Atalho sem teto não entra. Saída de filho, log de execução, eventos ainda não gravados e arquivo órfão têm os tetos da tabela.

## Paralelização

Na fase `execucao`, no máximo um ticket por rodada. Rodadas sem dependência correm juntas, cada uma na sua worktree (UC-EXE-01, UC-IMPL-01). Nas outras fases, no máximo um ticket da demanda. A falha de um não segura os outros. A varredura da branch base não segura o ciclo (UC-EXE-01).

Cada rodada tem worktree e diretório de compilação próprios. Commit, `worktree add`, `worktree remove`, fetch, branch e merge de um repositório entram numa fila, uma mutação por vez. A leitura do heatmap fica fora dessa fila (UC-PRJ-02).

Ao mesmo tempo, o padrão é 2 rodadas em `execucao` e o teto é 4. A seguinte espera o próximo ciclo. Antes de criar a worktree, o espaço livre cobre a maior worktree daquele projeto, ou 8 GB se nenhuma compilou, mais 4 GB. Sem esse espaço, o ciclo não inicia a rodada e o motivo fica no log.

Consulta, sessão em aberto e auditoria têm teto de 2 agentes interativos. A auditoria continua com um agente por sessão (UC-AUD-01).

Demanda concluída ou cancelada fica com zero worktrees (UC-DEM-03, UC-ENC-03).

## Memória

O RSS é o do processo da Fabriquinha. Agente, `cargo`, `rustc`, o scanner do SonarQube e o PostgreSQL ficam de fora. O teto de rodadas limita esses filhos.

Ocioso, até 64 MB. Em uso, com o poll do Início, um heatmap e quatro sessões vivas, até 256 MB.

A saída retida de um filho são os últimos 5 MB. Eventos de sessão ainda não gravados cabem em 8 MB; o restante já está em disco. Anexo copia em blocos de 1 MB. Texto e HTML aceitam 5 MB, imagem 20 MB, vídeo 500 MB.

## Disco

Banco, sessões, logs, evidências e auditorias ficam depois do encerramento e do cancelamento. A worktree sai.

O banco PostgreSQL cabe em 100 MB com cerca de 850 demandas. Blob fica fora do PostgreSQL, no sistema de arquivos.

Sessão: p95 até 8 MB, máximo 32 MB. Pasta de execução: p95 até 20 MB, com o log retido cortado no fim em 32 MB. Auditoria: uma pasta por mês; auditar de novo substitui a pasta, na ordem do tamanho das sessões daquele mês.

Hash sem referência e `.tmp` antigo saem na varredura.

## Métricas

| Métrica | Valor de referência | Justificativa |
|---|---|---|
| Decisão do tick e do Executar agora | p99 50 ms | Consulta de elegibilidade, antes da worktree. O tick não espera git nem agente. |
| Leitura de Início e de Projeto | p99 200 ms | A página relê a cada 5 s. Vale até 30 demandas em andamento. |
| Histórico: lista e prévia | p99 400 ms | 640 sessões hoje. A prévia usa o fim da conversa. A maior sessão tem 16 MB. |
| Métricas, 12 meses | p99 300 ms | O banco tem 13 MB. Trocar mês, card ou ordem não relê. |
| Evento do chat até o SSE | p99 100 ms | Só o trecho da Fabriquinha. O tempo do modelo fica no limite do agendador. |
| Heatmap | p99 3 s e até 128 MB de RAM | Cada abertura relê o git, sem fetch. O protótipo segura o `git log` inteiro em memória. |
| Checkout da worktree | p99 10 s | Checkout de até 500 MB, sem `target/` e sem `node_modules`. A preparação segue o limite de 30 min do comando. |
| Espera na fila git, fila ocupada por commit | p99 100 ms | Uma mutação git por repositório. A espera de um fetch pode ir até os 30 min do git, e o tick não entra nela. |
| RSS do serviço ocioso | até 64 MB | Processo da Fabriquinha, sem agente, `cargo`, `rustc`, scanner nem PostgreSQL. |
| RSS do serviço em uso | até 256 MB | Poll do Início, um heatmap e quatro sessões vivas. |
| Saída retida de um filho | últimos 5 MB | Já é o corte da preparação e da evidência texto no protótipo. |
| Eventos de sessão ainda não gravados | até 8 MB | Depois disso, só em disco. O protótipo acumula sem teto. |
| Cópia de anexo | blocos de 1 MB | Vídeo de 500 MB não entra na RAM. Texto e HTML 5 MB, imagem 20 MB, vídeo 500 MB. |
| Rodadas em `execucao` ao mesmo tempo | padrão 2, teto 4 | Uma rodada do `sincage-novo` já ocupa 16 GB, quase toda em `target/`. A seguinte espera o próximo ciclo. |
| Espaço livre antes de criar worktree | maior worktree do projeto, ou 8 GB, mais 4 GB | Sem esse espaço o ciclo não inicia a rodada. Cada rodada tem o próprio diretório de compilação. |
| Agentes interativos ao mesmo tempo | teto 2 | Consulta, sessão em aberto e auditoria. A auditoria continua com um agente por sessão. |
| Banco PostgreSQL, com cerca de 850 demandas | até 100 MB | Hoje são 13 MB para 85 demandas. Blob fica fora do PostgreSQL. |
| Arquivo de sessão | p95 8 MB, máximo 32 MB | Hoje o p95 é 5,6 MB e o máximo é 16 MB, em 642 sessões. |
| Pasta de uma execução | p95 20 MB, log retido 32 MB | Hoje duas execuções estão em 179 MB e 264 MB. O protótipo não corta esse log. |
| Pasta de auditoria | uma por mês, na ordem das sessões do mês | Hoje duas pastas, 21 MB e 66 MB. Auditar de novo substitui a pasta. |
| Worktrees de demanda concluída ou cancelada | 0 | UC-DEM-03 e UC-ENC-03 já pedem a limpeza. Hoje 22 demandas canceladas ainda têm pasta. |
