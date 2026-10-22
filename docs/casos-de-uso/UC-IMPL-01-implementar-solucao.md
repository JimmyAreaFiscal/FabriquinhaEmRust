# UC-IMPL-01 — Implementar a solução

| | |
|---|---|
| Ator | Agente, em sessão autônoma |
| Superfície | CLI, no ticket de implementação |

## Objetivo

O agente implementa um ticket de implementação da rodada, na worktree dela. A Fabriquinha confere testes e SonarQube e, com os dois aprovados, grava o commit. Todos os tickets de implementação da rodada concluem antes da revisão.

## Pré-condições

A demanda está em `execucao` e a rodada em `desenvolvimento`. O ticket está aberto. Os tickets da mesma rodada de que ele depende estão concluídos. As rodadas de que esta depende estão concluídas (UC-PLAN-02).

O agendador, ou o botão Executar agora, inicia a sessão. O modelo e o effort são o override do ticket, ou os da etapa "implementar" do projeto (UC-PLAN-02). O tempo limite da execução é o do agendador (UC-CFG-01).

Dentro da rodada corre uma execução por vez. O ticket que entra é o mais antigo aberto com as dependências concluídas. Rodadas sem dependência pendente correm em paralelo, cada uma na sua worktree.

## Contexto

O agente recebe o título, o tipo e a descrição da demanda, o título e os critérios da rodada, os caminhos deste ticket, o resumo e o fora de escopo do desenho, e os tickets da rodada já concluídos. O desenho completo ele lê na evidência da demanda.

## Fluxo principal

1. A Fabriquinha garante a worktree da rodada. Na primeira execução dela, cria a branch `fabriquinha/I-n-R-n` a partir de `fabriquinha/I-n` atualizada. Essa branch da demanda já contém o código das rodadas concluídas de que esta depende, porque o pull request de cada uma foi mergeado nela (UC-ENC-02). Em seguida roda a preparação do projeto (UC-CFG-02). A worktree fica fora do repositório e não é atualizada de novo durante a rodada.
2. O agente lê o desenho e o ticket e implementa na worktree, com os testes do que fez.
3. O agente encerra a resposta quando o ticket está implementado. O commit é da Fabriquinha.
4. A Fabriquinha confere os arquivos que o ticket alterou contra o tipo da rodada (abaixo).
5. Numa rodada de código, roda os testes, as fitness functions quando o projeto as tem, e o SonarQube, nessa ordem.
   Numa rodada de e2e, roda só o build e a execução e2e.
   Cada etapa só começa com a anterior aprovada.
6. Com a última etapa aprovada, a Fabriquinha commita e conclui o ticket.
7. Se ainda existe ticket de implementação da rodada por concluir, a rodada continua em `desenvolvimento`. Se este era o último, a rodada passa a `revisao` e nasce o ticket de revisão.

## Rodada de e2e e rodada de código

O corte vale para todo ticket de implementação, inclusive os que nascem da revisão, da sessão em aberto e do conflito.
Os arquivos alterados são os do diff da worktree contra o HEAD.

- Na rodada de e2e, todo arquivo alterado casa com um glob e2e do projeto (UC-CFG-02).
- Na rodada de código, nenhum arquivo alterado casa com um glob e2e.

Fora disso, o ticket vai a `falhou` com "a rodada de e2e só altera testes e2e" ou "a rodada de código não altera testes e2e", e a lista dos arquivos.
Nenhum comando roda e o commit não acontece.

Os testes e2e não rodam na rodada de código.
Enquanto houver rodada por concluir, eles podem falhar.
Eles precisam passar na revisão da demanda (UC-REV-02).

## Evidências

O agente não submete evidência. A Fabriquinha grava as desta etapa.

| Tipo | Aprovada quando |
|---|---|
| `execucao_testes` | Só na rodada de código. Cada comando de teste de cada linguagem saiu com código 0. |
| `matriz_fitness` | Só na rodada de código, quando o projeto tem fitness function. Cada comando saiu com código 0. A explicação de cada linha fica na revisão da rodada. |
| `relatorio_sonar` | Só na rodada de código. Nenhuma issue aberta nos arquivos alterados em relação ao HEAD. Issue que já existia num arquivo alterado também reprova. |
| `execucao_e2e` | Só na rodada de e2e. Uma linha por linguagem com bloco e2e, com o build e a execução. Cada build saiu com código 0 e cada execução saiu com 0 ou com o código de falha da linguagem (UC-CFG-02). Outro código, ou estouro, é erro do teste e reprova. |

Cada comando roda com limite de 30 minutos. Estouro de tempo conta como reprovação. A Fabriquinha roda todos os comandos do passo, mesmo depois de uma falha, e guarda comando, código de saída, estouro, duração e o log.

O comando de teste de cada linguagem roda no diretório correspondente da worktree: o caminho do cadastro, relativo à raiz do git, aplicado à worktree. Vale para todas as linguagens do projeto (UC-CFG-02).

O SonarQube usa a chave do projeto e `SONAR_HOST_URL` e `SONAR_TOKEN` do ambiente do serviço. A análise usa a configuração do servidor. Um `sonar-project.properties` da worktree não entra. Código de teste fica de fora pela exclusão do projeto no SonarQube.

## Commit

A Fabriquinha faz `git add -A` e commita com os hooks do repositório. O autor é a identidade git do repositório. A mensagem é:

```
<feat|fix|test>: <título do ticket> [I-n/T-m]

<descrição do ticket, cortada em 72 colunas>
```

`test` na rodada de e2e. Na rodada de código, `feat` quando a demanda é funcionalidade e `fix` quando é problema. Cada ticket de implementação tem um commit. Worktree sem alteração falha o ticket com "o ticket de implementação não alterou nenhum arquivo".

## Fluxo alternativo — reprovação ou falha

Testes reprovados, fitness reprovada, SonarQube com issue ou e2e com erro: a evidência fica gravada como reprovada, o commit não acontece e o ticket vai a `falhou`. SonarQube inacessível, scanner com falha ou host e token ausentes: o ticket vai a `falhou` com a mensagem, sem relatório aprovado. Hook que recusa o commit faz o mesmo, com a saída do hook. Preparação que falha encerra a execução com o log, antes do agente.

A falha não desfaz a worktree. O Auditor lê o erro no chat e reabre o ticket com uma instrução. A mesma sessão retoma quando o agendador, ou Executar agora, inicia a próxima execução. A instrução vem no início do prompt.

## Comandos

| Comando | Efeito |
|---|---|
| `projeto ver` | Lê o projeto. |
| `demanda ver I-n` | Lê a demanda. |
| `ticket ver T-n` | Lê o ticket. |
| `evidencia ver T-n <tipo>` | Lê uma evidência já gravada, como o desenho da demanda. |

A saída de sucesso é um JSON em uma linha. Esta etapa não tem comando de submissão.

## Métrica

Tokens e custo da sessão. A quantidade de alterações posteriores conta as linhas que este ticket escreveu e que um commit seguinte removeu ou reescreveu: outro ticket do projeto, ou um commit humano na branch base. A métrica não barra o ticket e cresce com o tempo. A conformidade do diff com os caminhos da rodada continua a métrica de UC-PLAN-01.

## Pós-condições

O ticket está concluído, com um commit na branch da rodada e as evidências desta etapa aprovadas. A rodada segue em `desenvolvimento` enquanto houver ticket de implementação por concluir. No último, ela fica em `revisao`, com o ticket de revisão aberto.
