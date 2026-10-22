# UC-REV-02 — Revisar a demanda

| | |
|---|---|
| Ator | Agente, em sessão autônoma |
| Superfície | CLI, no ticket de revisão da demanda |

## Objetivo

O agente revisa a demanda inteira em `fabriquinha/I-n` e faz a revisão adversarial. Aprovado, a demanda segue para o encerramento. Reprovado, ele abre rodadas de correção e a demanda volta à execução.

## Pré-condições

Toda rodada da demanda está concluída, com o pull request mergeado em `fabriquinha/I-n` (UC-ENC-02). A demanda passa a `revisao_demanda` e nasce este ticket. O mesmo vale depois de uma rodada de correção: quando a última delas conclui, este caso roda de novo.

O agendador, ou o botão Executar agora, inicia a sessão. O modelo e o effort são os da etapa "revisar a demanda" do projeto. O tempo limite é o do agendador (UC-CFG-01). A demanda tem uma execução por vez.

A Fabriquinha abre uma worktree temporária a partir de `fabriquinha/I-n` atualizada. Ela sai no fim da sessão.

## Contexto

O agente recebe o título, o tipo e a descrição da demanda, todos os critérios de aceite, o resumo, os arquivos previstos e o fora de escopo do desenho, e a árvore com a URL do pull request de cada rodada já mergeada.

## Fluxo principal

1. Antes do agente, se o projeto tem bloco e2e, a Fabriquinha roda o build e a execução e2e na worktree, com o mesmo limite de 30 minutos da implementação (UC-IMPL-01).
   O agente confere cada critério da demanda e grava a comprovação, com vídeo ou imagem.
2. Executa os testes de estresse e a análise de segurança na worktree e grava o resultado. Não há comando disso no projeto (UC-DEM-01).
3. Aprova, ou reprova e cria ao menos uma rodada de correção, com ticket de implementação.
4. A Fabriquinha conclui o ticket. O que estiver por commitar na worktree é descartado, e a worktree temporária é removida.
5. Aprovada, a demanda passa a `encerramento_demanda` e nasce o ticket de encerramento (UC-ENC-03). Reprovada, a demanda volta a `execucao`, com as rodadas de correção em `desenvolvimento`.

## Evidências

| Tipo | Quem grava | Conteúdo |
|---|---|---|
| `execucao_e2e` | Fabriquinha, antes do agente | Uma linha por linguagem com bloco e2e, com o build e a execução. Aprovada quando todos saem com código 0. Aqui o código de falha não é aceito. Sem bloco e2e no projeto, esta evidência não existe. |
| `comprovacao_criterios` | Agente | Exatamente os critérios da demanda, cada um uma vez. Cada item tem `id` `CA-n`, `atendido`, `comoVerificado` de 1 a 2.000 caracteres e ao menos um anexo. Aprovada quando todos estão atendidos. |
| `testes_estresse_seguranca` | Agente | `estresse`: ao menos um item, com `alvo` de 1 a 500 caracteres, `comoExecutado` de 1 a 2.000 e `passou`. `achados`: zero ou mais, cada um com `descricao` de 1 a 2.000 caracteres e `bloqueia`. Log em texto, até 5 MiB, quando houver. Aprovada quando todo estresse passou e nenhum achado bloqueia. |

O anexo sai da worktree ou de `$FABRIQUINHA_SAIDA`. Vídeo é WebM ou MP4, até 500 MiB. Imagem é PNG ou JPEG, até 20 MiB. A classe é a dos primeiros bytes. Cada nome citado no critério está entre os anexos. Submeter de novo o mesmo tipo substitui a evidência.

## Rodada de correção

A rodada nasce fora do gate de planejamento (UC-PLAN-02), marcada como correção. Tem título, ao menos um critério já existente na demanda e ao menos um ticket de implementação.
A exceção é a rodada de correção de e2e, sem critério.
Correção que altera teste e2e segue o corte de UC-PLAN-01 entre as rodadas criadas por esta revisão: uma rodada de correção de e2e, criada com `--e2e`, só com caminhos e2e, e as outras dependem dela. Citar um critério de novo não tira esse critério da rodada original.

O ticket tem título, descrição e os caminhos que altera. Um caminho é um caminho de `arquivosPrevistos` do desenho, ou um caminho que está em `fabriquinha/I-n` e não casa com nenhum glob do desenho, para a correção removê-lo. Dependência de rodada só aponta para outra rodada da mesma demanda, sem ciclo. Dependência de ticket só aponta para outro ticket da mesma rodada, sem ciclo. O modelo e o effort ficam vazios e saem da etapa "implementar".

O agente edita e remove só a rodada e o ticket que esta execução criou, e só enquanto a rodada não começou. A árvore aprovada no planejamento não se edita.

A rodada de correção segue UC-IMPL-01. A branch dela nasce de `fabriquinha/I-n`.

## Regras para concluir

- Aprovar exige as evidências aprovadas e nenhuma rodada de correção criada por esta revisão.
- Reprovar exige o motivo, de 1 a 2.000 caracteres, e ao menos uma rodada de correção com ticket de implementação criada por esta revisão.
- Uma decisão nova na mesma execução substitui a anterior.
- A combinação que não fecha falha o ticket, com a mensagem do que falta. O Auditor reabre com uma instrução, como em UC-IMPL-01.

## Comandos

| Comando | Efeito |
|---|---|
| `projeto ver` | Lê o projeto. |
| `demanda ver I-n` | Lê a demanda. |
| `ticket ver T-n` | Lê o ticket. |
| `evidencia schema <tipo>` | Devolve o JSON Schema do tipo. |
| `evidencia add <tipo> [--dados <arquivo.json\|->] [--arquivo <caminho>]...` | Grava ou substitui a evidência deste ticket. |
| `evidencia ver T-n <tipo>` | Lê uma evidência, como o desenho. |
| `rodada criar I-n --titulo <t> (--criterios <CA-1,CA-2> \| --e2e) [--deps <R-1,R-2>]` | Cria a rodada de correção. `--e2e` cria a de e2e, sem critérios. |
| `rodada editar R-n [--titulo <t>] [--criterios …] [--deps …]` | Altera a rodada que esta execução criou. |
| `rodada remover R-n` | Remove a rodada que esta execução criou, com os tickets dela. |
| `ticket criar R-n --titulo <t> --arquivos <glob,glob> [--descricao <arquivo\|->] [--deps <T-1,T-2>]` | Cria o ticket de implementação da correção. |
| `ticket editar T-n [--titulo <t>] [--arquivos …] [--descricao …] [--deps …]` | Altera o ticket que esta execução criou. |
| `ticket remover T-n` | Remove o ticket que esta execução criou. |
| `revisao decidir T-n aprovar\|reprovar [--motivo <texto>]` | Registra a decisão. Na reprovação o motivo é obrigatório. |

A saída de sucesso é um JSON em uma linha. Erro de regra sai no stderr com código 1. O agente não mergeia em `fabriquinha/I-n` nem na branch base.

## Métrica

Tokens e custo da sessão. A quantidade de exigências de alteração é o número de tickets de implementação nas rodadas de correção que esta revisão criou. A quantidade de bugs em arquivos revisados conta, com o tempo, as demandas do tipo problema do mesmo projeto que alteram um arquivo alterado por esta demanda em `fabriquinha/I-n`. Nenhuma das duas barra o ticket.

## Pós-condições

O ticket de revisão da demanda está concluído e a decisão está gravada. Aprovada, a demanda está em `encerramento_demanda`, com o ticket de encerramento aberto (UC-ENC-03). Reprovada, a demanda está em `execucao`, com as rodadas de correção em `desenvolvimento`. A branch base segue sem esta demanda.
