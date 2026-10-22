# UC-PLAN-01 — Planejar o trabalho

| | |
|---|---|
| Ator | Agente, em sessão autônoma |
| Superfície | CLI, no ticket de planejar o trabalho |

## Objetivo

O agente separa a demanda em rodadas e tickets. Cada rodada termina num pull request e tem vários tickets de implementação. O conjunto das rodadas resolve a demanda.
Quando o desenho altera testes e2e, a primeira rodada é a de e2e, só com eles, e as outras dependem dela.

## Pré-condições

A demanda está em `planejamento`, com o ticket aberto. Toda demanda da qual ela depende está concluída. O agendador, ou o botão Executar agora, inicia a sessão. O modelo e o effort são os da etapa "planejar trabalho" do projeto.

A Fabriquinha abre uma worktree só de leitura, a partir da branch base. Ela permanece durante a sessão, inclusive quando o planejamento volta reprovado, e é removida quando o planejamento é aprovado.

## Contexto

O agente recebe o título, o tipo e a descrição da demanda, os critérios de aceite e o desenho da solução: resumo, arquivos previstos, fora de escopo e o HTML.

## Fluxo principal

1. O agente lê o desenho e os critérios.
2. Cria uma ou mais rodadas. Cada uma tem título, os critérios que ela cumpre e, quando precisar, de quais outras rodadas da mesma demanda ela depende.
   A rodada de e2e é marcada e não tem critério.
3. Em cada rodada, cria um ou mais tickets de implementação, com título, descrição, os caminhos do desenho que aquele ticket altera e dependências entre tickets da mesma rodada.
4. Envia o planejamento.
5. A Fabriquinha confere a árvore, gera a evidência e leva a demanda a `aprovacao_planejamento`. O ticket de planejamento continua em execução até o Auditor decidir (UC-PLAN-02).

## Evidência

No envio, a Fabriquinha gera `arvore_planejamento` a partir do que foi gravado. O agente não escreve esse JSON. A evidência tem as duas partes do quadro:

- **Proposta de rodadas e tickets.** A demanda, as rodadas (`R-n`), com a de e2e marcada, os critérios de cada uma e os tickets (`T-n`), com título, dependências e os caminhos de cada ticket.
- **Lista de arquivos a alterar por rodada.** Cada caminho do desenho, os tickets em que ele entra e, por união desses tickets, as rodadas.

O anexo é um HTML com a mesma árvore, para o Auditor ler no gate. A cada ajuste do Auditor, a Fabriquinha regera essa evidência. A versão aprovada é a que vale.

## Rodada de e2e

Um caminho e2e é um caminho do desenho que casa com um glob e2e do projeto (UC-CFG-02).
A mudança nos testes e2e vem antes do código, numa rodada própria, com pull request próprio.
Quando o desenho tem caminho e2e:

- O envio tem exatamente uma rodada de e2e, criada com `--e2e`.
- Ela tem todos os caminhos e2e do desenho, e só eles.
- Ela não tem critério e não depende de outra rodada.
- Toda outra rodada da demanda depende dela.
- Nenhuma outra rodada tem caminho e2e.

Sem caminho e2e no desenho, não há rodada de e2e.
As outras rodadas são rodadas de código.
A Fabriquinha confere esse corte de novo no diff de cada ticket (UC-IMPL-01).

## Regras para o envio ser aceito

- Cada critério da demanda está em exatamente uma rodada de código.
- Cada caminho de `arquivosPrevistos` do desenho está em pelo menos uma rodada. O mesmo caminho pode entrar em mais de uma. Caminho que não está no desenho é recusado.
- Cada rodada tem ao menos um ticket de implementação.
- Dependência de rodada só aponta para outra rodada da mesma demanda, sem ciclo. Dependência de ticket só aponta para outro ticket da mesma rodada, sem ciclo.
- O corte da rodada de e2e, acima, é respeitado.
- O agente não escolhe modelo nem effort.

## Fluxo alternativo — planejamento reprovado

A reprovação chega na mesma sessão, com o motivo. O agente ajusta rodadas e tickets e envia de novo. O que já estava proposto permanece até ele mudar.

## Comandos

| Comando | Efeito |
|---|---|
| `rodada criar I-n --titulo <t> (--criterios <CA-1,CA-2> \| --e2e) [--deps <R-1,R-2>]` | Cria a rodada. `--e2e` cria a rodada de e2e, sem critérios. O tipo da rodada não muda depois. |
| `rodada editar R-n [--titulo <t>] [--criterios …] [--deps …]` | Altera a rodada ainda não iniciada. |
| `rodada remover R-n` | Remove a rodada, os tickets dela e a dependência nas outras. |
| `ticket criar R-n --titulo <t> --arquivos <glob,glob> [--descricao <arquivo\|->] [--deps <T-1,T-2>]` | Cria o ticket de implementação na rodada, com os caminhos que ele altera. |
| `ticket editar T-n [--titulo <t>] [--arquivos …] [--descricao …] [--deps …]` | Altera o ticket que esta execução criou. |
| `ticket remover T-n` | Remove o ticket que esta execução criou. |
| `ticket enviar T-n` | Envia a árvore à aprovação. O `T-n` é o ticket de planejamento. |

## Métrica

Tokens e custo da sessão. O percentual de conformidade do pull request com a lista de arquivos é medido na rodada, comparando o diff com os caminhos atrelados aqui.

## Pós-condições

A demanda está em `aprovacao_planejamento`, com a árvore enviada, à espera da decisão do Auditor.
