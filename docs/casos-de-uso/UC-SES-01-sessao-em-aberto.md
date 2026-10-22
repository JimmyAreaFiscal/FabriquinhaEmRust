# UC-SES-01 — Sessão em aberto

| | |
|---|---|
| Ator | Auditor, com o agente em sessão interativa |
| Superfície | Web, chat da sessão |

## Objetivo

O Auditor e o agente combinam o ajuste do pull request reprovado. O agente abre tickets de implementação na mesma rodada. O Auditor conclui a sessão, e a rodada volta ao desenvolvimento. O código não muda aqui.

## Pré-condições

A demanda está em `execucao` e a rodada em `sessao`. O ticket nasceu na reprovação (UC-ENC-02): o título é "Ajustar o pull request" e a descrição é o motivo. O pull request da rodada continua aberto contra `fabriquinha/I-n`.

A Fabriquinha abre o chat na hora, sem o agendador. A primeira mensagem traz o motivo e a URL do pull request. O modelo e o effort são os da etapa "sessão em aberto" (UC-CFG-02). A ociosidade é a do agendador (UC-CFG-01): o processo para, o chat continua à espera, e a mensagem seguinte retoma a mesma sessão.

## Contexto

O agente recebe o motivo, a URL do pull request, o título e os critérios da rodada, os caminhos declarados dela e o resumo do desenho.

## Fluxo principal

1. O Auditor conversa com o agente no chat.
2. O agente cria, na mesma rodada, um ticket de implementação para cada ajuste.
3. O Auditor conclui a sessão. O botão só conclui quando esta sessão criou ao menos um ticket de implementação.
4. A Fabriquinha descarta o que estiver por commitar na worktree. A branch `fabriquinha/I-n-R-n` fica como a implementação deixou.
5. Conclui o ticket da sessão e devolve a rodada a `desenvolvimento`, com os tickets novos abertos. Eles seguem UC-IMPL-01. A rodada só volta ao gate depois de outra revisão aprovada e de outro encerramento (UC-REV-01, UC-ENC-01).

## Tickets do ajuste

O agente cria o ticket na mesma rodada, com título, descrição e os caminhos que ele altera. Um caminho é um caminho já declarado na rodada, ou um caminho com `previsto` nulo no relatório da última revisão, para a correção removê-lo. Dependência só aponta para outro ticket da mesma rodada, sem ciclo. O modelo e o effort ficam vazios e saem da etapa "implementar".

O agente edita e remove só o ticket que esta sessão criou, e só enquanto está aberto. A árvore aprovada no planejamento não se edita (UC-PLAN-02).

## Regras

- A sessão não submete evidência e não faz commit, push nem merge.
- Concluir sem ticket de implementação falha o botão, com a mensagem do que falta. A sessão continua aberta.
- A sessão não vai a `falhou`. Cada reprovação deste pull request abre outra sessão, com outro ticket.

## Comandos

| Comando | Efeito |
|---|---|
| `projeto ver` | Lê o projeto. |
| `demanda ver I-n` | Lê a demanda. |
| `ticket ver T-n` | Lê o ticket. |
| `evidencia ver T-n <tipo>` | Lê uma evidência, como o desenho e o `arquivos_alterados`. |
| `ticket criar R-n --titulo <t> --arquivos <glob,glob> [--descricao <arquivo\|->] [--deps <T-1,T-2>]` | Cria o ticket de implementação do ajuste. |
| `ticket editar T-n [--titulo <t>] [--arquivos …] [--descricao …] [--deps …]` | Altera o ticket que esta sessão criou. |
| `ticket remover T-n` | Remove o ticket que esta sessão criou. |

A saída de sucesso é um JSON em uma linha. Erro de regra sai no stderr com código 1. Concluir a sessão é o botão do chat. O CLI não conclui.

## Métrica

Tokens e custo da sessão. A quantidade de ajustes é o número de tickets de implementação que esta sessão criou. Não barra a conclusão.

## Pós-condições

O ticket da sessão está concluído. A rodada está em `desenvolvimento`, com os tickets de implementação desta sessão abertos. O pull request segue aberto contra `fabriquinha/I-n`, e a branch da demanda segue sem este ajuste.
