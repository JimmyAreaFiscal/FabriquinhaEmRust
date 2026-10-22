# UC-ENC-01 — Encerrar a rodada

| | |
|---|---|
| Ator | Agente, em sessão autônoma |
| Superfície | CLI, no ticket de encerramento |

## Objetivo

O agente documenta a rodada para o histórico e para o revisor humano. A Fabriquinha monta o pull request, publica e só leva a rodada ao gate quando ele abre sem conflito e sem check falho.

## Pré-condições

A demanda está em `execucao` e a rodada em `encerramento`, com o ticket aberto. Esse ticket nasce quando a revisão aprova (UC-REV-01).

O agendador, ou o botão Executar agora, inicia a sessão. O modelo e o effort são os da etapa "encerrar a rodada" do projeto. O tempo limite é o do agendador (UC-CFG-01). A rodada tem uma execução por vez.

## Contexto

O agente recebe a rodada, o resumo do desenho, as revisões e os tickets concluídos. Se o pull request desta rodada já existe, recebe também o motivo da última reprovação.

## Fluxo principal

1. O agente lê o desenho, a última revisão e os tickets concluídos.
2. Grava a mensagem do pull request, a explicação do código e, quando a mudança aparece no cliente, o vídeo.
3. Encerra a resposta quando essas evidências estão aceitas. O push e o pull request são da Fabriquinha.
4. A Fabriquinha descarta o que estiver por commitar na worktree. A branch fica como a implementação deixou.
5. Gera o relatório de produtividade da rodada e publica o pull request da branch `fabriquinha/I-n-R-n` contra `fabriquinha/I-n`. Se o pull request já existe, troca o título e o corpo inteiros.
6. Confere conflito e checks. Sem conflito e sem check falho, conclui o ticket e leva a rodada a `aprovacao_pr`.

## Evidências do agente

Submeter de novo o mesmo tipo substitui. O anexo sai da worktree ou de `$FABRIQUINHA_SAIDA`.

| Tipo | Conteúdo |
|---|---|
| `mensagem_pr` | `titulo` de 1 a 72 caracteres, no imperativo. `oQueMudou`, até 3.000 caracteres. `decisoesNaoObvias` e `naoFeito`, itens de até 1.000; lista vazia vira "Nenhuma." e "Nada declarado.". `naoPrevistos`: exatamente um item por arquivo com `previsto` nulo no `arquivos_alterados` da última revisão concluída, cada um com motivo de 1 a 2.000 caracteres. `comoRevisar`: ao menos um passo, de até 500 caracteres, na ordem de leitura. `naoVerificado`: itens de até 1.000; vazio vira "Nada.". `reversao`, quando houver, até 2.000 caracteres. Sem anexo. |
| `aula_codigo` | Ao menos um trecho, na ordem de leitura. Cada um tem o caminho como no `arquivos_alterados`, `inicio` e `fim` inclusive, com no máximo 60 linhas, `oQueFaz` e `decisaoDoDesenho` de até 1.000 caracteres, `porQue` de até 2.000, e `alternativaDescartada` opcional de até 1.000. O agente manda só o JSON. A Fabriquinha lê o código no git, na `head` da última revisão, e gera o HTML. |
| `video_demonstracao` | Só quando a mudança aparece no cliente. Quem decide é o agente. `roteiro` de 1 a 2.000 caracteres e um vídeo, WebM ou MP4, até 500 MiB. |

A Fabriquinha aceita o trecho quando o arquivo foi criado ou alterado pela rodada, não é binário, o intervalo cabe no arquivo e toca ao menos uma linha que o diff adicionou ou mudou. Fora disso, a submissão é recusada e a mensagem diz as linhas válidas.

## O que a Fabriquinha publica

O título é `<feat|fix|test>: <titulo> [I-n/R-n]`. `test` para a rodada de e2e. Na rodada de código, `feat` para funcionalidade e `fix` para problema. A frase é a do agente.

O corpo é Markdown, sempre com estas seções, nesta ordem:

| # | Seção | Fonte |
|---|---|---|
| 1 | Por quê | Tipo, título e descrição da demanda, o lugar da rodada na árvore e os critérios dela. |
| 2 | O que mudou | `oQueMudou`. |
| 3 | Decisões não óbvias | `decisoesNaoObvias`. |
| 4 | Fora do escopo | `foraDeEscopo` do desenho e `naoFeito`. |
| 5 | Arquivos alterados × desenho | Tabela do `arquivos_alterados` da última revisão: arquivo, status, linhas e o caminho previsto, ou "não previsto" com o motivo. |
| 6 | Como revisar | `comoRevisar`, numerado. |
| 7 | Evidências | Testes e SonarQube do último ticket de implementação da rodada, ou a execução e2e na rodada de e2e. Fitness da última revisão, quando o projeto tem fitness function, com o resultado e a explicação. Critérios da rodada, com atendido, como foi verificado e o anexo. |
| 8 | Não verificado | `naoVerificado`. |
| 9 | Reversão | `reversao`, quando o agente informou. Sem ela, a seção diz "Nada.". |
| 10 | Autoria e custo | Por etapa da rodada: modelos, execuções, tokens, custo e reprovações. |

O corpo cita a explicação do código, o vídeo e o relatório pelo número da demanda, o tipo e o nome do arquivo. GitHub aceita 65.536 caracteres e Azure DevOps 4.000. Acima do limite, a Fabriquinha condensa nesta ordem e para quando couber: a seção 5 vira a contagem mais a lista dos não previstos; a seção 7 vira uma linha por evidência; a seção 10 vira os totais; o que ainda passar é cortado com "… corpo completo em I-n, evidência pr_aberto". As seções do agente só são condensadas depois das geradas. O corpo completo fica anexado a `pr_aberto`.

`pr_aberto` guarda a hospedagem, o identificador, a URL, a branch, a base, o `head` e se o corpo publicado foi condensado.

O relatório de produtividade é o registro local da rodada, em JSON e HTML. Traz tokens, custo, execuções, falhas, retrabalho, exigências da revisão e as decisões já gravadas. A nota do Auditor ainda não existe.

## Publicação e conferência

A Fabriquinha faz o push da branch e, se a rodada ainda não tem pull request, cria. Antes de criar, lista os pull requests da branch. Um que não esteja em `pr_aberto` falha o ticket com "pull request criado fora da Fabriquinha", a URL e o pedido para fechá-lo. O corpo leva uma marca invisível com a demanda e o ticket. Se a plataforma criou o pull request e o registro falhou, a execução seguinte adota esse pull request pela marca.

A conferência lê o conflito com `fabriquinha/I-n` e os checks:

- Com conflito, devolve na hora.
- Sem conflito, espera check em andamento e lê de novo a cada 30 segundos.
- Lista vazia depois de 2 minutos é pull request sem CI e segue limpo.
- Aos 60 minutos com check ainda em andamento, o ticket falha e a mensagem pede para acompanhar na plataforma e executar de novo.
- Valor que a Fabriquinha não reconhece falha o ticket.
- Falha do `gh` ou do `az` falha o ticket com a saída do comando.

Na rodada de e2e, a conferência é a mesma.
Cabe ao CI do projeto não rodar os testes e2e num pull request que só altera testes e2e.

## Fluxo alternativo — conflito ou check falho

O ticket de encerramento conclui mesmo assim. A rodada volta a `desenvolvimento` com um ticket de implementação aberto pela Fabriquinha. No conflito, o título pede para resolver o conflito com a branch da demanda `fabriquinha/I-n` e a descrição traz a URL e os comandos de fetch e merge sem commit. No check falho, o título lista os checks e a descrição traz a URL de cada um. O modelo e o effort saem da etapa "implementar". Esse ticket segue UC-IMPL-01. A rodada só volta ao encerramento depois de uma revisão aprovada (UC-REV-01).

## Comandos

| Comando | Efeito |
|---|---|
| `projeto ver` | Lê o projeto. |
| `demanda ver I-n` | Lê a demanda. |
| `ticket ver T-n` | Lê o ticket. |
| `evidencia schema <tipo>` | Devolve o JSON Schema do tipo. |
| `evidencia add <tipo> [--dados <arquivo.json\|->] [--arquivo <caminho>]...` | Grava ou substitui a evidência deste ticket. |
| `evidencia ver T-n <tipo>` | Lê uma evidência já gravada. |

A saída de sucesso é um JSON em uma linha. Erro de regra sai no stderr com código 1. O agente não cria, edita nem faz merge de pull request.

## Métrica

Tokens e custo da sessão. A nota de 1 a 5 do Auditor entra quando ele decide o pull request, no caso seguinte. Não barra este ticket.

## Pós-condições

O ticket de encerramento está concluído. Sem conflito e sem check falho, a rodada está em `aprovacao_pr`, com o pull request aberto, à espera do Auditor. Com conflito ou check falho, a rodada está em `desenvolvimento`, com o ticket de correção aberto.
