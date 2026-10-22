# UC-ENC-02 — Decidir o pull request da rodada

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página da demanda, gate `aprovacao_pr` |

## Objetivo

O Auditor dá a nota de qualidade da rodada e aprova ou reprova o pull request dela contra `fabriquinha/I-n`. Aprovado, a Fabriquinha mergeia esse pull request na branch da demanda. Reprovado, abre a sessão em aberto. A branch base não muda neste gate.

## Pré-condições

A demanda está em `execucao` e a rodada em `aprovacao_pr`, com o pull request aberto contra `fabriquinha/I-n` (UC-ENC-01). A decisão é no web. O CLI não aprova.

## O que o gate mostra

Um gate por rodada em `aprovacao_pr`.

O link do pull request e o estado lido na hospedagem. O vídeo de demonstração, quando a rodada tem essa evidência. Sem ela, o aviso de que a mudança não aparece no cliente. Quatro números do relatório de produtividade da rodada: custo, tokens, execuções, com as falhas, e reprovações. A explicação do código. O corpo completo guardado em `pr_aberto`. A comprovação dos critérios da última revisão. A matriz de fitness com as explicações, quando o projeto tem fitness function.

O Auditor não reescreve esses artefatos.

## Fluxo principal — aprovar

1. O Auditor escolhe a nota, de 1 a 5, e aprova. O botão só habilita com a nota.
2. A Fabriquinha mergeia o pull request em `fabriquinha/I-n` com merge commit. Os commits dos tickets da rodada permanecem nessa branch.
3. Com o merge feito, grava a aprovação e a nota. A rodada sai de `aprovacao_pr` e fica concluída.
4. Uma rodada que depende desta já pode nascer de `fabriquinha/I-n`, que agora tem o código desta (UC-IMPL-01).
5. Se ainda há rodada desta demanda por concluir, a demanda continua em `execucao`. Se esta era a última, a demanda passa a `revisao_demanda` e nasce o ticket de revisão da demanda (UC-REV-02). O pull request de `fabriquinha/I-n` contra a branch base fica para o encerramento da demanda.

Se o merge não acontece, a decisão não é gravada e a rodada permanece em `aprovacao_pr`. O gate mostra a mensagem da plataforma.

## Fluxo alternativo — reprovar

1. O Auditor escolhe a nota, escreve o motivo e reprova. O botão exige a nota e o motivo, de 1 a 2.000 caracteres.
2. A Fabriquinha grava a reprovação com a nota e o motivo. Não há merge.
3. A rodada passa a `sessao`. Nasce o ticket da sessão em aberto, com o título "Ajustar o pull request" e o motivo na descrição. O modelo e o effort são os da etapa "sessão em aberto" (UC-CFG-02).
4. O pull request continua aberto. O que a sessão faz no código está em UC-SES-01.

## Regras

- A decisão só cabe com a rodada em `aprovacao_pr`. Fora disso, o gate informa que a decisão já foi tomada ou que o pull request ainda não chegou.
- A nota é um inteiro de 1 a 5. Ela registra o feedback e não escolhe o botão.
- Cada passagem da rodada por este gate grava uma decisão. Uma rodada reprovada e, depois do ajuste, aprovada fica com as duas notas.
- A IA e o agente não decidem este gate.

## Métrica

A nota de 1 a 5. É o feedback humano da qualidade anunciado em UC-ENC-01. Não barra a decisão.

## Pós-condições

A decisão está gravada, com a nota. Aprovada, a rodada está concluída e o pull request está mergeado em `fabriquinha/I-n`. A branch base segue sem esta rodada. Reprovada, a rodada está em `sessao`, com o ticket da sessão em aberto, e o pull request segue aberto.
