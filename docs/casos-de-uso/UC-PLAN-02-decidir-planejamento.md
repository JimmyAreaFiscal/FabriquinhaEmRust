# UC-PLAN-02 — Decidir o planejamento

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página da demanda, gate `aprovacao_planejamento` |

## Objetivo

O Auditor aprova a divisão em rodadas e tickets, ou a devolve ao mesmo agente com o motivo. Aprovada, a árvore congela e as rodadas podem começar.

## Pré-condições

O agente enviou o planejamento (UC-PLAN-01). A demanda está em `aprovacao_planejamento`. A sessão está aguardando o Auditor. A decisão é no web. O CLI não aprova.

## O que o gate mostra

A árvore regerada: cada rodada com título, critérios e dependências, e os tickets com título, descrição, dependências e os caminhos que cada um altera. Critério ainda sem rodada fica em destaque. Caminho do desenho que não está em nenhum ticket, e caminho de ticket que o desenho não propôs, também ficam em destaque.
A rodada de e2e aparece marcada.
Quebra do corte da rodada de e2e (UC-PLAN-01) também fica em destaque: caminho e2e fora dela, rodada de código que não depende dela, ou caminho e2e sem rodada de e2e.

Em cada ticket, um seletor de modelo e, ao lado, o effort. Vazio nos dois usa o padrão da etapa "implementar" do projeto. Valor preenchido vale só para aquele ticket e é marcado como override.

## Ajustes antes de decidir

O Auditor pode criar, editar e remover rodadas e tickets, inclusive critérios, caminhos e dependências, com as mesmas regras do envio. Os caminhos ficam no ticket. A lista da rodada é a união dos tickets dela. Cada ajuste regera a `arvore_planejamento`.

Até aprovar, o Auditor também pode incluir ou remover dependências da demanda, sem ciclo e sem autorreferência.

## Fluxo principal — aprovar

1. O botão só habilita quando cada critério está em exatamente uma rodada, cada rodada tem ao menos um ticket e o conjunto dos caminhos de todos os tickets de implementação é igual ao conjunto de `arquivosPrevistos` do desenho da demanda e o corte da rodada de e2e é respeitado.
2. A Fabriquinha para o processo do agente se ele ainda estiver de pé, grava a aprovação, conclui o ticket de planejamento, leva a fase a `execucao` e congela a árvore.
3. A worktree de leitura do planejamento é removida.
4. A Fabriquinha cria e publica a branch `fabriquinha/I-n` a partir de `origin/<branch base>`. Se a publicação falha, a aprovação não é gravada e o gate mostra a saída do git.
5. Cada rodada nasce em desenvolvimento, com os tickets de implementação abertos. Uma rodada só começa quando as rodadas de que ela depende estão concluídas.

## Fluxo alternativo — reprovar

1. O botão só habilita com o motivo, de 1 a 2.000 caracteres.
2. A Fabriquinha grava a reprovação, devolve a fase a `planejamento`, tira a marca de enviado e mantém a árvore.
3. A mesma sessão autônoma retoma com a mensagem "O Auditor reprovou o planejamento. Motivo: …". O agente ajusta e envia de novo (UC-PLAN-01).

## Depois de aprovado

Rodada e ticket dessa árvore não se criam, editam ou removem mais. A rodada de correção, que nasce numa revisão posterior, fica fora deste gate. A branch `fabriquinha/I-n` está publicada a partir da branch base. Os pull requests das rodadas entram nela (UC-ENC-02).
