# UC-HIS-01 — Ver o histórico

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Histórico |

## Objetivo

O Auditor filtra as sessões terminadas e lê a prévia de uma delas. **Abrir o chat** leva à sessão terminada, só leitura (UC-EXE-02). **Conversar sobre** leva à consulta do ticket (UC-EXE-03). **Exportar** baixa o Markdown da sessão. Auditar um mês inteiro fica em UC-AUD-01.

## Pré-condições

O Auditor abre o Histórico no web. Entra na lista a sessão cujo estado é `terminada`.

## Fluxo principal

1. O Auditor abre o Histórico.
2. A Fabriquinha lista as sessões terminadas, da mais recente para a mais antiga.
3. Havendo sessão, a mais recente começa selecionada.
4. A prévia dela aparece ao lado da lista.
5. O Auditor restringe por projeto, demanda ou etapa.
6. A contagem e a lista passam a mostrar só o que casa com os filtros.
7. O Auditor escolhe uma linha.
8. A prévia troca para essa sessão, sem sair da página.

## O que a página mostra

O título é Histórico. Ao lado, a frase "sessões terminadas, da mais recente para a mais antiga".

| Filtro | O que aceita |
|---|---|
| Projeto | Um projeto cadastrado, ou todos. |
| Demanda | `I-n` ou só o número. Vazio não restringe. |
| Etapa | Todas, ou planejar, implementar, revisar a rodada, encerrar a rodada, sessão em aberto, revisar a demanda. |

A contagem fica ao lado dos filtros. Uma sessão conta "1 sessão". As demais contam "N sessões".

A lista agrupa pelo dia em que a sessão ficou terminada, no fuso do navegador.

| Dia | Rótulo |
|---|---|
| O dia de hoje | Hoje |
| O dia anterior | Ontem |
| Outro dia do ano corrente | `DD/MM` |
| Dia de outro ano | `DD/MM/AAAA` |

| Na linha | Conteúdo |
|---|---|
| Hora | A hora em que a sessão ficou terminada, `HH:MM`. |
| Demanda | `I-n`. |
| Ticket | `T-n`. |
| Título | O título da demanda. Sem título, "sem título". |
| Etapa e modo | A etapa e o modo. |
| Resultado | Concluída, falha, abortada ou cancelada. |
| Custo | O custo da sessão em `US$`, com duas casas. |

O modo é autônoma, interativa ou consulta. Sem sessão no filtro, a página diz "Nenhuma sessão com esses filtros."

A prévia fica ao lado. Enquanto chega, diz "Carregando a sessão…".

| Fato | O que mostra |
|---|---|
| Cabeçalho | `I-n · T-n` e o título da demanda. |
| Projeto | O nome do projeto. |
| Etapa | A etapa e o modo. |
| Resultado | O mesmo resultado da linha. |
| Terminada | O dia, a hora e a duração. |
| Custo | O custo e o modelo. |
| Mensagens | O número de mensagens do Auditor e do agente. |

O fim da conversa mostra os três últimos pedaços. O pedaço do Auditor leva "você". O pedaço do agente leva "agente". A chamada de ferramenta leva o nome, os argumentos e o resultado. Sem pedaço, o fim diz "sem mensagens".

Os botões da prévia são **Abrir o chat**, **Conversar sobre** e **Exportar**.

## Regras

A ordem, o dia, a hora da linha e o fato Terminada usam o instante em que a sessão ficou terminada.

A duração soma o tempo de cada execução da sessão que já tem fim. Execução sem fim soma zero.

O custo soma o uso de todas as execuções da sessão.

O modelo é o da última execução, quando ela fixou um. Sem modelo na execução, vale o da última resposta do agente. Sem resposta, a prévia diz "modelo padrão do pi". O nome sai como `provedor / modelo`.

O prompt que a Fabriquinha montou fica fora da contagem e fora do fim. O pedaço de texto usa só o bloco de texto. Raciocínio não entra. Texto vazio não entra. O texto sai em Markdown. HTML da mensagem não vira marcação.

A chamada fica recolhida. Os argumentos mostram o comando, quando a chamada tem um. Sem comando, mostram o caminho. Sem os dois, mostram o JSON. O resumo do resultado é ✓ ou ✗ com o número de linhas. Saída vazia diz "sem saída". Sem resultado ainda, diz "sem resultado ainda". Saída com mais de 10 linhas ganha **ver tudo**. **ver tudo** abre a caixa `Saída de <nome>` com a saída inteira.

| Última execução | Resultado |
|---|---|
| Não falhou | concluída |
| Falhou com "abortada pelo Auditor" | abortada |
| Falhou com "demanda cancelada pelo Auditor" | cancelada |
| Falhou com outro erro | falha |

Os três filtros valem juntos. A demanda casa com `I-n` ou com o número, com ou sem hífen, em qualquer caixa. Texto que não é número deixa a lista vazia, sem consulta. O texto da demanda vale sem espaços nas pontas.

A sessão escolhida permanece quando ainda está na lista. Fora da lista, a mais recente do filtro toma o lugar. Os filtros e a sessão escolhida permanecem entre as visitas à página. A lista muda quando o Auditor abre a página ou mexe num filtro.

O mês ao lado da contagem fica fora destes filtros. Auditar o mês é UC-AUD-01.

**Abrir o chat** abre essa sessão terminada, só leitura (UC-EXE-02).

**Conversar sobre** pede a consulta do ticket `T-n`. A Fabriquinha abre a sessão que essa consulta devolve (UC-EXE-03). Pedido que falha mantém o Auditor no Histórico. O erro mostrado é o do servidor.

**Exportar** baixa o Markdown da sessão. O arquivo se chama `historico-T-n.md`. A sessão de consulta usa `historico-T-n-consulta-<sessão>.md`. O conteúdo é o do anexo de UC-EXE-02.

## Pós-condições

A sessão lida continua terminada. A demanda e o ticket permanecem como estavam. **Exportar** entrega o arquivo ao Auditor. **Conversar sobre** deixa uma sessão nova (UC-EXE-03).
