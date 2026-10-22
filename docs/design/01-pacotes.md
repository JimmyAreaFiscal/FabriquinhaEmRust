# Design — Pacotes

Este documento fixa como o código da Fabriquinha em Rust se organiza.
As entidades, os serviços, as interfaces e as sequências vêm nos documentos seguintes de `docs/design/`.

## Decisões

| # | Decisão | Motivo |
|---|---|---|
| P1 | Workspace Cargo com duas crates: `dominio` e `fabriquinha`. | O núcleo sem IO (atributos de qualidade, Manutenibilidade) fica garantido pelas dependências da crate, não por disciplina. |
| P2 | Uma crate só para o IO e os casos de uso. | Há uma instância e um binário. Separar banco, git e web em crates não protege nenhuma regra dos casos. |
| P3 | Um módulo por caso de uso em `casos/`. O tipo se chama `{Ação}CasoDeUso` e o arquivo `{ação}.rs`, como `DecidirDemandaCasoDeUso` em `decidir_demanda.rs`. O comentário do módulo cita o UC. | "O módulo que decide cita o UC." A chamada lê como a ação: `decidir_demanda::aprovar`. |
| P4 | Trait só onde há mais de uma implementação real. | Hospedagem é GitHub ou Azure DevOps. O pi falso dos testes é outro binário em `$FABRIQUINHA_PI`, sem trait. |
| P5 | `templates/` com o system prompt, as skills e a lista de tools de cada processo pi, embutidos no binário. | A versão dos templates é sempre a do binário. |
| P5.1 | As tools ficam no pacote pi `pi-fabriquinha`, em `pacote-pi/`, fora do binário. | O pi só carrega extensões em TypeScript. O pacote se instala no pi global com `pi install`, que também instala as dependências dele. |
| P6 | O CLI do agente e o da IA externa nunca abrem o banco. Falam com o serviço por HTTP. | O serviço é o único dono do banco, da fila git e dos processos pi. |
| P7 | PostgreSQL em Docker para os dados; o sistema de arquivos para blobs, sessões, logs e worktrees. | Escolha do Auditor. O serviço roda no Ubuntu, fora do Docker. A conexão vem de `$FABRIQUINHA_BANCO` (UC-CFG-05). |
| P8 | Frontend em Angular, em `web/`. O build fica embutido no binário e o serviço o entrega em `/static`. | Um artefato só para instalar, e o web sempre na mesma versão da API. Único código fora de Rust, além das tools do pi (P5.1). |
| P9 | Sem login, token ou cookie. O serviço só escuta em `127.0.0.1` (UC-CFG-05). | Uso local, sem exposição na rede. |

## Árvore

```
fabriquinha/
  Cargo.toml                  workspace
  crates/
    dominio/                  lib pura
      src/
        lib.rs
        projeto.rs            linguagens, fitness, modelo e effort por etapa
        demanda.rs            fases, dependências, cancelamento em cascata, arquivamento
        rodada.rs             fases da rodada, correção, dependências
        ticket.rs             etapas, status, origem, override de modelo
        arvore.rs             validação do planejamento (critérios × rodadas × caminhos)
        elegibilidade.rs      quem o tick dispara (UC-EXE-01)
        sessao.rs             estado, espera e resultado da sessão
        evidencia/            tipos, schemas e regras de aprovação
        custo.rs              uso, preço, soma por etapa
        metricas.rs           agregações mensais (UC-MET-01)
        heatmap.rs            árvore e calor a partir dos commits já lidos
        pr.rs                 corpo do pull request e condensação
    fabriquinha/              binário
      src/
        main.rs
        casos/                um módulo por UC
        banco/                sqlx, migrações, repositórios
        arquivos/             blobs de evidência, sessões, logs, auditorias, varredura
        git/                  fila por repositório, worktree, commit, merge, leitura do heatmap
        hospedagem/           GitHub (gh) e Azure DevOps (az)
        sonar/                scanner e leitura das issues
        agente/               processo pi: lançamento, RPC, tempo limite, ociosidade, eventos
        comandos/             testes, fitness e preparação, com teto de saída e de tempo
        agendador/            tick, Executar agora, varredura diária da branch base
        web/                  axum: /api, SSE, estáticos embutidos
        cli/                  clap: comandos do Auditor e cliente HTTP do agente
      templates/              embutido no binário (P5)
  pacote-pi/                  pacote pi `pi-fabriquinha`, em TypeScript (P5.1)
  web/                        Angular
```

## Crate `dominio`

Tipos e regras dos casos de uso, sem IO.
Elegibilidade, gate, fase e cancelamento se testam aqui, sem git, sem agente e sem HTTP, em menos de 30 segundos.

Dependências permitidas: `serde`, `serde_json`, `thiserror`, `time` e `jsonschema`.
Qualquer outra entra por decisão registrada neste documento.

O Cargo não impede `std::fs`, `std::process` e `std::net`, que vêm da biblioteca padrão.
Uma fitness function na crate `fabriquinha` falha quando esses caminhos aparecem em `crates/dominio`.

A crate não conhece Postgres, axum nem o pi.
Os módulos de `casos/` leem o estado, chamam o domínio para decidir e gravam o resultado.

## Crate `fabriquinha`

### Direção das dependências

```
web ─┐
     ├─> casos ─> dominio
cli ─┘      │
            ├─> banco, arquivos, git, hospedagem, sonar, agente, comandos
agendador ──┘
```

| Módulo | Pode usar | Não pode usar |
|---|---|---|
| `casos` | `dominio` e todos os módulos de IO | `web`, `cli` |
| `web` | `casos`, `dominio` | `banco`, `git` e os demais módulos de IO diretamente |
| `agendador` | `casos`, `dominio` | IO diretamente |
| `cli` (cliente do agente) | `dominio` para os tipos, cliente HTTP | `banco`, `casos` e qualquer módulo de IO |
| módulos de IO | `dominio` | `casos`, `web`, `cli` |

As regras da tabela viram uma fitness function sobre os `use` de cada módulo.

### `casos/`

O arquivo tem o nome da ação do caso, e o tipo é `{Ação}CasoDeUso` (P3).
Cada módulo de caso que sobe pi declara o template que usa (ver `templates/`).

| Arquivo | Tipo | Caso | Template pi |
|---|---|---|---|
| `consultar_inicio.rs` | `ConsultarInicioCasoDeUso` | UC-INI-01 | — |
| `configurar_agendador.rs` | `ConfigurarAgendadorCasoDeUso` | UC-CFG-01 | — |
| `cadastrar_projeto.rs` | `CadastrarProjetoCasoDeUso` | UC-CFG-02 | — |
| `editar_projeto.rs` | `EditarProjetoCasoDeUso` | UC-CFG-03 | — |
| `remover_projeto.rs` | `RemoverProjetoCasoDeUso` | UC-CFG-04 | — |
| `entrar_no_servico.rs` | `EntrarNoServicoCasoDeUso` | UC-CFG-05 | — |
| `registrar_demanda.rs` | `RegistrarDemandaCasoDeUso` | UC-DEM-01 | — (a IA é externa) |
| `decidir_demanda.rs` | `DecidirDemandaCasoDeUso` | UC-DEM-02 | — |
| `cancelar_demanda.rs` | `CancelarDemandaCasoDeUso` | UC-DEM-03 | — |
| `arquivar_demandas.rs` | `ArquivarDemandasCasoDeUso` | UC-DEM-04 | — |
| `planejar_trabalho.rs` | `PlanejarTrabalhoCasoDeUso` | UC-PLAN-01 | `planejar` |
| `decidir_planejamento.rs` | `DecidirPlanejamentoCasoDeUso` | UC-PLAN-02 | — |
| `disparar_trabalho.rs` | `DispararTrabalhoCasoDeUso` | UC-EXE-01 | o do ticket disparado |
| `acompanhar_execucao.rs` | `AcompanharExecucaoCasoDeUso` | UC-EXE-02 | o da sessão |
| `conversar_sobre_sessao.rs` | `ConversarSobreSessaoCasoDeUso` | UC-EXE-03 | `consulta` |
| `ajustar_pull_request.rs` | `AjustarPullRequestCasoDeUso` | UC-SES-01 | `sessao-em-aberto` |
| `implementar_solucao.rs` | `ImplementarSolucaoCasoDeUso` | UC-IMPL-01 | `implementar` |
| `revisar_rodada.rs` | `RevisarRodadaCasoDeUso` | UC-REV-01 | `revisar-rodada` |
| `revisar_demanda.rs` | `RevisarDemandaCasoDeUso` | UC-REV-02 | `revisar-demanda` |
| `encerrar_rodada.rs` | `EncerrarRodadaCasoDeUso` | UC-ENC-01 | `encerrar-rodada` |
| `decidir_pull_request.rs` | `DecidirPullRequestCasoDeUso` | UC-ENC-02 | — |
| `encerrar_demanda.rs` | `EncerrarDemandaCasoDeUso` | UC-ENC-03 | sem agente; `resolver-conflito` no fluxo de conflito |
| `consultar_projeto.rs` | `ConsultarProjetoCasoDeUso` | UC-PRJ-01 | — |
| `ver_heatmap.rs` | `VerHeatmapCasoDeUso` | UC-PRJ-02 | — |
| `ver_metricas.rs` | `VerMetricasCasoDeUso` | UC-MET-01 | — |
| `ver_historico.rs` | `VerHistoricoCasoDeUso` | UC-HIS-01 | — |
| `auditar_historico.rs` | `AuditarHistoricoCasoDeUso` | UC-AUD-01 | `auditoria` |

### Módulos de IO

| Módulo | Responsabilidade | Casos e atributos |
|---|---|---|
| `banco` | Pool `sqlx` do Postgres, migrações embutidas, consultas por agregado. Transação por operação de caso. | Todos. Banco até 100 MB com cerca de 850 demandas. |
| `arquivos` | Raiz `$FABRIQUINHA_ROOT`. Blob de evidência endereçado por hash, cópia em blocos de 1 MB, `.tmp` e hash sem referência na varredura. Sessões, logs de execução, pastas de auditoria. | UC-CFG-04, UC-HIS-01, UC-AUD-01; Disco e Memória. |
| `git` | Fila de mutações por repositório, em memória. Worktree e diretório de compilação por rodada, espaço livre antes de criar. Leitura do heatmap fora da fila. | UC-PLAN-02, UC-IMPL-01, UC-ENC-*, UC-DEM-03, UC-PRJ-02. |
| `hospedagem` | Trait com GitHub e Azure DevOps: criar e atualizar pull request, ler conflito e checks, mergear, fechar. Limite do corpo por plataforma. | UC-CFG-02, UC-ENC-01, UC-ENC-02, UC-ENC-03, UC-DEM-03. |
| `sonar` | Rodar o scanner com `SONAR_HOST_URL` e `SONAR_TOKEN` e ler as issues dos arquivos alterados. | UC-IMPL-01, UC-ENC-03. |
| `agente` | Sobe o pi com os argumentos do template, fala RPC, aplica tempo limite e ociosidade, mata o grupo do processo, guarda até 8 MB de eventos por gravar e publica no SSE. Teto de 2 agentes interativos. | UC-EXE-02, UC-EXE-03, UC-SES-01, UC-AUD-01. |
| `comandos` | Testes, fitness e preparação: limite de 30 minutos, saída retida dos últimos 5 MB, log cortado em 32 MB. | UC-IMPL-01, UC-REV-01, UC-CFG-02. |

### `agendador`

O tick de minuto em minuto, o Executar agora e a varredura diária da branch base.
Decide com `dominio::elegibilidade` e entrega o ticket ao caso dele.
O tick não espera git nem agente (p99 de 50 ms).
No máximo 2 rodadas em `execucao` por padrão, teto 4.

### `web`

Axum em `127.0.0.1:$FABRIQUINHA_PORTA`.
A porta é a trava de instância única: o serviço a ocupa antes de abrir o banco (UC-CFG-05).
Sem login: o web e o CLI usam as mesmas rotas em `/api`.
O serviço recusa `Host` diferente de `127.0.0.1:<porta>` e pedido de escrita com `Origin` de outro site (UC-CFG-05).
Eventos do chat por SSE.
O Angular compilado sai em `/static`, e os demais `GET` devolvem o `index.html`.

### Web embutido

O Angular não roda no servidor.
`ng build` transforma o projeto `web/` em arquivos estáticos: HTML, JavaScript e CSS.
Quem executa esse JavaScript é o navegador do Auditor.

O `build.rs` da crate `fabriquinha` roda `npm run build` em `web/` quando os fontes dele mudam.
Sem `web/node_modules`, o build falha e pede `npm ci` em `web/`.
A macro `include_dir!`, a mesma dos templates, copia `web/dist` para dentro do binário.
Em execução, o axum entrega esses arquivos a partir da memória do próprio binário.

No desenvolvimento do web, `ng serve` recarrega a tela a cada mudança e encaminha `/api` para o serviço em `127.0.0.1:4317`.

### `cli`

Um binário, `fabriquinha`, com dois grupos.

| Grupo | Comandos | Fala com |
|---|---|---|
| Auditor | `servidor`, `abrir` | `servidor` sobe o serviço; `abrir` lê `<raiz>/web.url`. |
| Agente e IA externa | `demanda`, `projeto`, `rodada`, `ticket`, `evidencia`, `revisao` | O serviço, por HTTP em `127.0.0.1`. Nunca o banco. |

O grupo do agente transforma argumentos em pedido HTTP e a resposta em uma linha JSON, com os códigos de saída 0, 1 e 2 dos casos.
Serviço fora do ar é erro com a mensagem que manda subir `fabriquinha servidor`.

## `templates/`

Embutidos no binário.
A cada subida, o serviço apaga `<raiz>/templates` e o extrai de novo, porque o pi só aceita caminhos de arquivo.

```
templates/
  skills/                     compartilhadas
    artefato-comprovacao-criterios/SKILL.md
    artefato-explicacao-fitness/SKILL.md
    artefato-mensagem-pr/SKILL.md
    artefato-aula-codigo/SKILL.md
    artefato-video-demonstracao/SKILL.md
    artefato-testes-estresse-seguranca/SKILL.md   a criar
  planejar/
    system_prompt.md
    template.toml
  implementar/
  revisar-rodada/
  encerrar-rodada/
  sessao-em-aberto/
  revisar-demanda/
  resolver-conflito/
  consulta/
  auditoria/
```

O `template.toml` lista as skills do template, as tools do `pi-fabriquinha` e as ferramentas nativas do pi que ficam desligadas.
Uma skill ou tool citada que não existe falha um teste da crate, não a subida.

`artefato-testes-estresse-seguranca` ainda não existe no protótipo.
Ela é um objetivo a criar, junto com o schema da evidência `testes_estresse_seguranca` (UC-REV-02).

| Template | Modo | Skills | Tools | Nativas desligadas |
|---|---|---|---|---|
| `planejar` | autônomo | — | `guarda`, `perguntar` | `ask_user_question` |
| `implementar` | autônomo | — | `guarda`, `perguntar`, `navegador` | `ask_user_question` |
| `revisar-rodada` | autônomo | `comprovacao-criterios`, `explicacao-fitness` | `guarda`, `perguntar`, `navegador` | `ask_user_question` |
| `encerrar-rodada` | autônomo | `mensagem-pr`, `aula-codigo`, `video-demonstracao` | `guarda`, `perguntar`, `navegador` | `ask_user_question` |
| `revisar-demanda` | autônomo | `comprovacao-criterios`, `testes-estresse-seguranca` | `guarda`, `perguntar`, `navegador` | `ask_user_question` |
| `resolver-conflito` | autônomo | — | `guarda`, `perguntar` | `ask_user_question` |
| `sessao-em-aberto` | interativo | — | `guarda`, `perguntar` | `ask_user_question` |
| `consulta` | consulta | — | `guarda`, `perguntar` | `ask_user_question`, `edit`, `write`, `bash`, `powershell` |
| `auditoria` | consulta | — | `guarda` (só `scout`) | `edit`, `write`, `bash`, `powershell` |

O modelo e o effort não estão no template.
Saem da etapa do projeto ou do override do ticket (UC-CFG-02, UC-PLAN-02).

## `pacote-pi/`

O pacote pi `pi-fabriquinha`, com as extensões que viram tools do agente.

```
pacote-pi/
  package.json                name pi-fabriquinha, keywords pi-package, versão igual à do binário
  extensions/
    guarda.ts                 recusa gh e az que criam, alteram ou mergeiam pull request
    perguntar.ts              diálogo com o Auditor no lugar de ask_user_question (UC-EXE-02)
    navegador.ts              Chromium headless dirigido pelo agente (a criar)
```

### `navegador`

Abre um Chromium headless e dá ao agente as tools de usar a página: abrir um endereço, clicar, digitar, rolar, esperar, ler o texto e tirar print.
Cada tool devolve o print e o endereço atual.
O agente grava vídeo da navegação para as evidências de comprovação e de demonstração (UC-REV-01, UC-REV-02, UC-ENC-01), em `$FABRIQUINHA_SAIDA`.
O endereço vem do agente, como o app que ele subiu na worktree; o projeto não tem link de produto (UC-CFG-02).
O Chromium sai junto com a sessão do pi.

Usa Playwright, nas `dependencies` do pacote.
O Chromium do Playwright se instala uma vez com `npx playwright install chromium`.
Ele é filho do pi e fica fora do RSS da Fabriquinha, como o agente.

A tool do protótipo, ligada à Simulação, serve de ponto de partida e é reescrita.

O Auditor instala uma vez no pi global, sem carregar nada nas sessões dele:

```json
{ "source": "<caminho>/pacote-pi", "extensions": [], "skills": [] }
```

Com o filtro vazio, as sessões pessoais do Auditor não ganham a `guarda` nem a `perguntar`.
A Fabriquinha acha o caminho do pacote em `pi list` e passa cada tool do template com `-e <caminho>/extensions/<tool>.ts`.

Na subida, o serviço confere que `pi-fabriquinha` está instalado e que a versão é a do binário.
Sem o pacote, ou com outra versão, o serviço sai com erro e diz o comando `pi install` que falta.

## Testes

| Nível | Onde | O que usa |
|---|---|---|
| Unidade | `crates/dominio` | Nada de IO. Menos de 30 segundos. |
| Integração | `crates/fabriquinha/tests` | Postgres de teste em Docker, repositórios git temporários, pi falso em `$FABRIQUINHA_PI`. |
| Fitness | `crates/fabriquinha/tests/fitness` | Domínio sem IO, direção das dependências, `cli` sem `banco`, templates completos. |
| E2E | `web/e2e` | Serviço real, navegador, pi falso. |
