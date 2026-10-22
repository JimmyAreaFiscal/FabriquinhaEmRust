# Design — Serviços

Este documento fixa onde mora cada regra e cada fluxo dos casos de uso.
Os pacotes estão em `01-pacotes.md` e as entidades em `02-dominio.md`.

## Decisões

| # | Decisão | Motivo |
|---|---|---|
| S1 | O domínio não tem serviços. As regras são métodos das entidades ou funções dos módulos de `dominio`. | Uma regra com um só dono fica no dono. Um "serviço" de um chamador só é indireção. |
| S2 | Cada caso de uso é um `{Ação}CasoDeUso` em `casos/` (P3). | O caso é a regra. |
| S3 | Só dois serviços de aplicação são compartilhados: `executar_etapa` e `publicar_pr`. | São os únicos fluxos repetidos em mais de um caso. |
| S4 | Só dois serviços contínuos: Agendador e GerenteDeAgentes. | São as únicas coisas que vivem sozinhas no tempo: o relógio e os processos pi. |
| S5 | Fila git, eventos e inicialização não são serviços. | São um `Mutex` por repositório, um canal `broadcast` e uma função de `main.rs`. |
| S6 | As alterações posteriores ficam numa projeção: tabela derivada, refeita pela varredura e descartável. | Acumulam com o tempo; recalcular do git a cada leitura não cabe nos 300 ms de UC-MET-01. Não é entidade (D5). |

## Domínio

Funções puras, sem IO (`01-pacotes.md`, crate `dominio`).

| Módulo | Regras | Casos |
|---|---|---|
| `projeto.rs` | Construtores que validam slug, caminhos e a URL do `origin`, que define a hospedagem. | UC-CFG-02, UC-CFG-03 |
| `demanda.rs` | Transições de fase. Dependências sem ciclo. Cancelamento em cascata: recebe o grafo e devolve as demandas a cancelar e o novo status de cada ticket. Próxima fase quando a última rodada conclui. | UC-DEM-01 a 04, UC-ENC-02, UC-REV-02, UC-ENC-03 |
| `rodada.rs` | Transições de fase. Próxima fase quando o último ticket de implementação conclui. Conclusão da revisão: aprovar ou reprovar só com as evidências e os tickets exigidos. | UC-IMPL-01, UC-REV-01, UC-ENC-01, UC-ENC-02, UC-SES-01 |
| `ticket.rs` | Transições de status. Quem pode editar ou remover. Modelo e effort efetivos: override, etapa do projeto ou padrão do harness. | todos |
| `sessao.rs` | Estado, motivo da espera e resultado da sessão terminada: concluída, falha, abortada ou cancelada. | UC-EXE-02, UC-HIS-01, UC-AUD-01 |
| `arvore.rs` | Cada critério em uma rodada só. Caminhos dos tickets iguais aos do desenho. Função `tem_ciclo`, usada também por demanda, rodada e ticket. | UC-PLAN-01, UC-PLAN-02, UC-REV-02, UC-SES-01 |
| `elegibilidade.rs` | O ticket que o tick dispara em cada rodada e em cada demanda, e o motivo de quem não entra. O Executar agora usa a mesma função. | UC-EXE-01 |
| `evidencia/` | Schema e aprovação de cada tipo. Substituição (D11). Casamento do diff com os globs: `previsto` e `previstosNaoTocados`. Trecho válido da `aula_codigo`. Classe do anexo pelos primeiros bytes. | UC-DEM-01, UC-IMPL-01, UC-REV-01, UC-REV-02, UC-ENC-01, UC-ENC-03 |
| `custo.rs` | Tokens vezes `PrecoModelo`, em micro-dólares. Uso externo duplicado. | UC-DEM-01, todos com agente |
| `pr.rs` | Mensagem de commit. Título e corpo do PR, com a marca invisível e a condensação até o limite da hospedagem. Leitura da conferência: limpo, conflito, check falho, esperando ou desconhecido. | UC-IMPL-01, UC-ENC-01, UC-ENC-03 |
| `metricas.rs` | Agregações do mês e do período, média, variação. Contagem das alterações posteriores a partir dos diffs lidos. | UC-MET-01, UC-INI-01, UC-PRJ-01, UC-ENC-01, UC-AUD-01 |
| `heatmap.rs` | Árvore, totais, calor e o PR de cada commit. | UC-PRJ-02 |

## Aplicação

### Padrão de um caso

Cada operação de um `{Ação}CasoDeUso` recebe um `&Contexto` e segue esta ordem:

1. Carrega o agregado pelo `banco`.
2. Chama o domínio, que decide ou devolve o erro da regra.
3. Grava a decisão numa transação.
4. Faz o IO externo: git, hospedagem, pi.

Duas operações invertem os passos 3 e 4, porque os casos pedem que nada seja gravado se o IO falhar:

| Operação | Caso | Regra |
|---|---|---|
| `decidir_planejamento::aprovar` | UC-PLAN-02 | Publica `fabriquinha/I-n` antes de gravar a aprovação. |
| `decidir_pull_request::aprovar` | UC-ENC-02 | Mergeia o PR antes de gravar a decisão. |

O `Contexto` é criado uma vez em `main.rs` e compartilhado.

| Campo | O que é |
|---|---|
| `banco` | Pool do PostgreSQL. |
| `raiz` | `$FABRIQUINHA_ROOT`. |
| `git` | Módulo `git`, com o `Mutex` de cada repositório. |
| `agentes` | Endereço do GerenteDeAgentes. |
| `precos` | Catálogo de `precos.json`. |
| `relogio` | O instante atual; nos testes, um relógio fixo. |

### Casos

| Caso | Tipo | Usa |
|---|---|---|
| UC-INI-01 | `ConsultarInicioCasoDeUso` | `metricas` |
| UC-CFG-01 | `ConfigurarAgendadorCasoDeUso` | — |
| UC-CFG-02 | `CadastrarProjetoCasoDeUso` | `projeto`, `git`, `hospedagem`, `agente` (lista de modelos) |
| UC-CFG-03 | `EditarProjetoCasoDeUso` | como o cadastro |
| UC-CFG-04 | `RemoverProjetoCasoDeUso` | `arquivos`, `git` |
| UC-CFG-05 | `EntrarNoServicoCasoDeUso` | `cli` (`servidor`, `abrir`) |
| UC-DEM-01 | `RegistrarDemandaCasoDeUso` | `demanda`, `evidencia`, `custo` |
| UC-DEM-02 | `DecidirDemandaCasoDeUso` | `demanda`, GerenteDeAgentes |
| UC-DEM-03 | `CancelarDemandaCasoDeUso` | `demanda`, GerenteDeAgentes, `git`, `hospedagem` |
| UC-DEM-04 | `ArquivarDemandasCasoDeUso` | `demanda` |
| UC-PLAN-01 | `PlanejarTrabalhoCasoDeUso` | `arvore`, `executar_etapa` |
| UC-PLAN-02 | `DecidirPlanejamentoCasoDeUso` | `arvore`, GerenteDeAgentes, `git` |
| UC-EXE-01 | `DispararTrabalhoCasoDeUso` | `elegibilidade`, `executar_etapa` |
| UC-EXE-02 | `AcompanharExecucaoCasoDeUso` | GerenteDeAgentes |
| UC-EXE-03 | `ConversarSobreSessaoCasoDeUso` | GerenteDeAgentes |
| UC-SES-01 | `AjustarPullRequestCasoDeUso` | `executar_etapa`, `git` |
| UC-IMPL-01 | `ImplementarSolucaoCasoDeUso` | `executar_etapa`, `comandos`, `sonar`, `git` |
| UC-REV-01 | `RevisarRodadaCasoDeUso` | `executar_etapa`, `comandos`, `evidencia` |
| UC-REV-02 | `RevisarDemandaCasoDeUso` | `executar_etapa`, `git` |
| UC-ENC-01 | `EncerrarRodadaCasoDeUso` | `executar_etapa`, `publicar_pr` |
| UC-ENC-02 | `DecidirPullRequestCasoDeUso` | `hospedagem` |
| UC-ENC-03 | `EncerrarDemandaCasoDeUso` | `publicar_pr`, `git`; `executar_etapa` no conflito |
| UC-PRJ-01 | `ConsultarProjetoCasoDeUso` | `metricas` |
| UC-PRJ-02 | `VerHeatmapCasoDeUso` | `git` (leitura fora da fila), `heatmap` |
| UC-MET-01 | `VerMetricasCasoDeUso` | `metricas`, projeção de alterações posteriores |
| UC-HIS-01 | `VerHistoricoCasoDeUso` | `sessao`, `arquivos` |
| UC-AUD-01 | `AuditarHistoricoCasoDeUso` | `metricas`, `arquivos`, GerenteDeAgentes |

A montagem do Início, do trilho do projeto, do Markdown da sessão e do `indice.md` fica no caso que a usa: cada uma tem um chamador só.

### `executar_etapa`

O fluxo comum das sete etapas com agente (UC-PLAN-01, UC-IMPL-01, UC-REV-01, UC-REV-02, UC-ENC-01, UC-SES-01 e o conflito de UC-ENC-03).

1. Grava a execução, com o modelo, o effort e o tempo limite vigentes, e leva o ticket a `em_execucao`.
2. Garante a worktree da etapa, com a preparação do projeto.
3. Preenche o `system_prompt.md` do template com o contexto da etapa e a instrução de retomada, se houver.
4. Pede ao GerenteDeAgentes que suba o pi com o template.
5. Quando o agente encerra, devolve o controle ao caso, que confere as evidências e conclui ou falha o ticket.

O caso entrega ao `executar_etapa` só o que é dele: o template, o contexto e o que fazer no fim.

### `publicar_pr`

As regras comuns de UC-ENC-01 e UC-ENC-03.

1. Push da branch pela fila git.
2. Lista os PRs da branch. Adota o que tem a marca invisível; falha com o que foi criado fora da Fabriquinha.
3. Cria o PR, ou troca o título e o corpo inteiros.
4. Confere conflito e checks a cada 30 segundos, até 60 minutos.
5. Devolve a leitura de `pr::conferencia` e grava `pr_aberto` ou `pr_demanda`.

## Contínuos

### Agendador

Uma tarefa tokio que acorda a cada minuto.

1. Com o agendador ligado e o intervalo vencido, chama `disparar_trabalho`, que decide pela `elegibilidade` em até 50 ms e entrega cada ticket ao `executar_etapa` sem esperar.
2. Para cada projeto sem varredura há 24 horas, dispara a varredura da branch base sem esperar. A varredura busca `origin/<branch base>`, conta as alterações posteriores e refaz a projeção (S6).
3. Uma vez por dia, apaga o hash sem referência e o `.tmp` antigo.

A falha de um projeto ou de um ticket vai ao log e não segura os outros (UC-EXE-01).

### GerenteDeAgentes

Uma tarefa tokio, dona única dos processos pi.
Os casos só pedem: subir, mandar mensagem, responder diálogo, abortar e parar.

| Regra | Caso |
|---|---|
| Sobe o pi com a configuração global do Auditor, que traz provedores como o `pi-claude-bridge` e o `subagent` da auditoria, mais as skills e as tools do template. | `01-pacotes.md`, UC-AUD-01 |
| Tempo limite conta só o tempo ativo. Ao estourar, `SIGTERM` no grupo, 10 segundos, `SIGKILL`. | UC-CFG-01, UC-EXE-02 |
| Ociosidade para o processo interativo e a consulta; a mensagem seguinte retoma a mesma sessão. | UC-CFG-01, UC-EXE-03, UC-SES-01, UC-AUD-01 |
| No máximo 2 agentes interativos ao mesmo tempo. | atributos de qualidade |
| Guarda até 8 MB de eventos ainda não gravados e os publica num canal `broadcast`, que o SSE lê. | UC-EXE-02, atributos de qualidade |
| Retém os últimos 5 MB da saída de cada filho. | atributos de qualidade |

## Inicialização

Uma função de `main.rs`, nesta ordem (UC-CFG-05):

1. Ocupa a porta.
2. Conecta ao PostgreSQL e aplica as migrações.
3. Confere o `pi-fabriquinha` instalado e extrai os templates.
4. Manda `SIGKILL` ao pi que ficou no ar e leva o ticket autônomo a `falhou` com "serviço reiniciado" (UC-EXE-02, UC-EXE-03, UC-AUD-01).
5. Sobe o Agendador e o GerenteDeAgentes.
6. Grava `web.url`.

## Projeção das alterações posteriores

Tabela derivada, fora do domínio (S6).
Cada linha: o commit que reescreveu, o ticket cujas linhas foram reescritas, quantas linhas e se o commit é de ticket ou humano.

A varredura acrescenta as linhas dos commits novos desde o cursor do projeto.
Apagar a tabela e zerar o cursor faz a próxima varredura refazer tudo a partir do git.
