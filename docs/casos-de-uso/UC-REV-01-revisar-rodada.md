# UC-REV-01 — Revisar a rodada

| | |
|---|---|
| Ator | Agente, em sessão autônoma |
| Superfície | CLI, no ticket de revisão |

## Objetivo

O agente revisa o que a rodada implementou. Aprovado, a rodada segue para o encerramento. Reprovado, ele abre tickets de implementação na mesma rodada, um por exigência, e a rodada volta ao desenvolvimento.

## Pré-condições

A demanda está em `execucao` e a rodada em `revisao`, com o ticket de revisão aberto. Esse ticket nasce quando o último ticket de implementação da rodada conclui (UC-IMPL-01).

O agendador, ou o botão Executar agora, inicia a sessão. O modelo e o effort são os da etapa "revisar a rodada" do projeto. O tempo limite é o do agendador (UC-CFG-01). A rodada tem uma execução por vez.

## Contexto

O agente recebe o título, o tipo e a descrição da demanda, o título, os critérios e os caminhos da rodada, o resumo e o fora de escopo do desenho, os tickets da rodada já concluídos, e o resumo das duas evidências que a Fabriquinha coleta antes dele.

## Fluxo principal

1. Antes do agente, a Fabriquinha coleta o relatório de arquivos alterados. Se a rodada é de código e o projeto tem fitness function, coleta a matriz, rodando cada comando na worktree da rodada, com o mesmo limite de 30 minutos da implementação (UC-IMPL-01).
2. O agente confere cada critério da rodada e grava a comprovação, com vídeo ou imagem.
   A rodada de e2e não tem critério nem comprovação: o agente revisa se os testes e2e exercitam os critérios da demanda.
3. Com matriz, explica cada linha.
4. Aprova, ou reprova e cria um ticket de implementação para cada exigência.
5. A Fabriquinha conclui o ticket de revisão. A branch fica como a implementação deixou: o que estiver por commitar na worktree é descartado, e as evidências permanecem.
6. Aprovada, a rodada passa a `encerramento` e nasce o ticket de encerramento. Reprovada, a rodada volta a `desenvolvimento`, com os tickets novos abertos. Quando eles concluírem, nasce outro ticket de revisão e este caso roda de novo.

## Evidências

| Tipo | Quem grava | Conteúdo |
|---|---|---|
| `arquivos_alterados` | Fabriquinha, antes do agente | O diff da rodada, de `merge-base(fabriquinha/I-n, HEAD)` até `HEAD`. Cada arquivo traz caminho, status (`A`, `M`, `D` ou `R`), linhas adicionadas e removidas, e `previsto`: o caminho declarado da rodada que casou, ou nulo. Renomeação casa pelo caminho novo e pelo antigo. `previstosNaoTocados` lista os caminhos declarados da rodada com que nenhum arquivo casou. Arquivo binário deixa a contagem de linhas nula. |
| `matriz_fitness` | Fabriquinha, antes do agente | Uma linha por fitness function do projeto: nome, descrição, comando, código de saída, estouro, duração e log. Aprovada quando todas saem com código 0. Sem fitness function no projeto, esta evidência não existe. |
| `explicacao_fitness` | Agente | Uma explicação por linha da matriz atual, de 1 a 1.000 caracteres, no nome da fitness function. Sem anexo. Submeter de novo substitui. Sem matriz, não se submete. |
| `comprovacao_criterios` | Agente | Exatamente os critérios da rodada, cada um uma vez. Cada item tem `id` `CA-n`, `atendido`, `comoVerificado` de 1 a 2.000 caracteres e ao menos um anexo. Aprovada quando todos estão atendidos. |

O anexo sai da worktree ou de `$FABRIQUINHA_SAIDA`. Vídeo é WebM ou MP4, até 500 MiB. Imagem é PNG ou JPEG, até 20 MiB. A classe é a dos primeiros bytes. Cada nome citado no critério está entre os anexos. Submeter de novo o mesmo tipo substitui a evidência.

Os caminhos declarados da rodada são os do planejamento aprovado, a parte do desenho atrelada a ela (UC-PLAN-01). A Fabriquinha casa o arquivo do diff com o glob declarado.

## Tickets de uma reprovação

O agente cria o ticket na mesma rodada, com título, descrição e os caminhos que ele altera. Um caminho é um caminho já declarado na rodada, ou um caminho com `previsto` nulo no relatório, para a correção removê-lo. Dependência só aponta para outro ticket da mesma rodada, sem ciclo. O modelo e o effort ficam vazios e saem da etapa "implementar".

O agente edita e remove só o ticket que esta execução criou, e só enquanto está aberto. A árvore aprovada no planejamento não se edita aqui (UC-PLAN-02).

## Regras para concluir

- Aprovar exige a comprovação aprovada, na rodada de código, nenhuma fitness reprovada e nenhum ticket de implementação criado por esta revisão. Com matriz, a explicação cobre exatamente as linhas da matriz atual.
- Reprovar exige o motivo, de 1 a 2.000 caracteres, e ao menos um ticket de implementação criado por esta revisão.
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
| `evidencia ver T-n <tipo>` | Lê uma evidência, como `arquivos_alterados` e `matriz_fitness`. |
| `ticket criar R-n --titulo <t> --arquivos <glob,glob> [--descricao <arquivo\|->] [--deps <T-1,T-2>]` | Cria o ticket de implementação da reprovação. |
| `ticket editar T-n [--titulo <t>] [--arquivos …] [--descricao …] [--deps …]` | Altera o ticket que esta execução criou. |
| `ticket remover T-n` | Remove o ticket que esta execução criou. |
| `revisao decidir T-n aprovar\|reprovar [--motivo <texto>]` | Registra a decisão. Na reprovação o motivo é obrigatório. |

A saída de sucesso é um JSON em uma linha. Erro de regra sai no stderr com código 1.

## Métrica

Tokens e custo da sessão. A quantidade de exigências de alteração é o número de tickets de implementação que esta revisão criou. A quantidade de bugs em arquivos revisados conta, com o tempo, as demandas do tipo problema do mesmo projeto que alteram um arquivo alterado por esta rodada. Nenhuma das duas barra o ticket.

## Pós-condições

O ticket de revisão está concluído e a decisão está gravada. Aprovada, a rodada está em `encerramento`, com o ticket de encerramento aberto. Reprovada, a rodada está em `desenvolvimento`, com os tickets de implementação desta revisão abertos.
