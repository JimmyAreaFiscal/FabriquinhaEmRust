# UC-ENC-03 — Encerrar a demanda

| | |
|---|---|
| Ator | Fabriquinha |
| Superfície | Ticket de encerramento da demanda, sem gate |

## Objetivo

A Fabriquinha publica o pull request de `fabriquinha/I-n` contra a branch base, faz o merge e limpa as branches e as worktrees desta demanda. O Auditor não participa deste merge. O CI/CD da branch base segue na hospedagem.

## Pré-condições

A revisão da demanda aprovou (UC-REV-02). A demanda está em `encerramento_demanda`, com o ticket aberto. Toda rodada está concluída e mergeada em `fabriquinha/I-n`.

O agendador, ou o botão Executar agora, dispara o ticket. Nesta etapa não há agente. O tempo limite é o do agendador (UC-CFG-01). A demanda tem uma execução por vez.

## Fluxo principal

1. A Fabriquinha gera a árvore de rodadas, pull requests, critérios e custos.
2. Publica o pull request de `fabriquinha/I-n` contra `origin/<branch base>`. Se ele já existe, troca o título e o corpo inteiros.
3. Confere conflito e checks com as mesmas regras de UC-ENC-01. A base desta conferência é a branch base.
4. Sem conflito e sem check falho, faz o merge com merge commit. Os commits dos tickets permanecem na branch base.
5. Com o merge feito, remove as worktrees das rodadas e as temporárias desta demanda. Apaga, no local e no `origin`, as branches `fabriquinha/I-n-R-*` desta demanda e `fabriquinha/I-n`.
6. Grava a demonstração de limpeza. Com as listas vazias, conclui o ticket, leva a demanda a `concluida`. A mudança de fase guarda a data.

Um pull request que a plataforma já mergeou entra no passo 5. A adoção de um pull request criado fora do registro segue a marca de UC-ENC-01.

## O que a Fabriquinha publica

O título é `<feat|fix>: <título da demanda> [I-n]`. `feat` para funcionalidade e `fix` para problema.

O corpo é Markdown, nesta ordem: por quê (tipo, título, descrição e critérios), rodadas com a URL de cada pull request mergeado, critérios da última revisão da demanda, custos por fase e o total. Acima do limite da hospedagem, vale a condensação de UC-ENC-01.

`pr_demanda` guarda a hospedagem, o identificador, a URL, a branch, a base, o `head` e se o corpo foi condensado. O corpo completo fica anexado.

## Evidências

As duas são da Fabriquinha. O agente não as escreve.

| Tipo | Conteúdo |
|---|---|
| `arvore_prs_criterios_custos` | Cada rodada com título, se é correção, a URL do pull request e quando foi mergeada, e os critérios dela. Os critérios da última revisão da demanda, com atendido e como foi verificado. O custo em dólar da demanda, do planejamento, de cada rodada, da revisão da demanda e o total. Um HTML com a mesma árvore. |
| `demonstracao_limpeza` | Só depois do merge feito. Quatro listas: worktrees, pastas de worktree no disco, branches locais e branches remotas desta demanda. Um texto com a saída dos comandos. Aprovada quando as quatro listas estão vazias. |

Evidências, sessões e logs ficam.

A `branch base` local avança até `origin/<branch base>` só por fast-forward, no mesmo passo da limpeza. Quando não dá, a limpeza conclui e a resposta traz o aviso com o motivo e a saída do git.

## Fluxo alternativo — conflito

O merge não acontece. Não há limpeza nem demonstração. O ticket de encerramento não conclui. Nasce o ticket "Resolver o conflito de merge", com a mensagem da plataforma na descrição. O modelo e o effort são os da etapa "resolver conflito de merge" (UC-CFG-02).

O agente integra `origin/<branch base>` em `fabriquinha/I-n`. A Fabriquinha roda testes, fitness quando o projeto tem, e SonarQube, e commita, como em UC-IMPL-01. Com o commit, esse ticket conclui e o encerramento roda de novo.

## Fluxo alternativo — check falho ou limpeza incompleta

Check falho não faz merge e não limpa. O ticket de encerramento falha com a URL de cada check. Executar agora tenta a conferência de novo.

Limpeza que ainda deixa worktree, pasta ou branch desta demanda não desfaz o merge. A Fabriquinha tenta remover de novo. O que sobrar falha o ticket, com o caminho e o comando. A execução seguinte vê o pull request mergeado e só limpa.

## Métrica

Tokens e custo da sessão de conflito, quando ela roda. Não barra o encerramento. Este merge não recebe nota do Auditor.

## Pós-condições

A demanda está `concluida`. O pull request de `fabriquinha/I-n` está mergeado na branch base. As branches e as worktrees desta demanda não existem mais. O histórico permanece.
