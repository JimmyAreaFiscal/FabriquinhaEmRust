# UC-DEM-02 — Decidir a demanda

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, página da demanda, gate `aprovacao_demanda` |

## Objetivo

O Auditor aprova a demanda antes de qualquer planejamento, ou a recusa. Aprovada, nasce o trabalho de planejar. Recusada, a demanda se encerra com o motivo e não volta à IA.

## Pré-condições

A demanda foi enviada pela IA externa (UC-DEM-01) e está na fase `aprovacao_demanda`. O ticket de demanda está marcado como enviado. O Auditor age no web. O CLI não tem comando de aprovação.

## Fluxo principal — aprovar

1. O Auditor abre a demanda.
2. O gate mostra o título, o tipo, a descrição, a tabela de critérios de aceite (`CA-n`, dado, quando, então), o desenho (resumo, arquivos previstos, fora de escopo e o HTML) e o custo realizado por ferramenta, com tokens e valor.
3. O Auditor aprova.
4. A Fabriquinha grava a decisão de aprovação, conclui o ticket de demanda, leva a fase a `planejamento` e abre um ticket de planejar o trabalho.
5. Esse ticket fica disponível para o agendador. O planejamento só começa quando toda demanda da qual esta depende está concluída.

## Fluxo alternativo — recusar

1. No mesmo gate, o Auditor escreve o motivo e recusa.
2. O botão de recusar só habilita com o motivo preenchido. A tela avisa que a demanda e as que dependem dela serão canceladas, e que a demanda não volta à IA.
3. A Fabriquinha grava a decisão de recusa com o motivo e leva a demanda à fase `cancelada`. A mudança de fase guarda a data e a fase de onde saiu. Nada é apagado.
4. O ticket de demanda em execução passa a cancelado. Tickets abertos ou que falharam passam a fechados.
5. Toda demanda que depende desta, direta ou indiretamente, em qualquer projeto, é cancelada na mesma operação. As demandas das quais esta depende não mudam.
6. Execução em andamento numa demanda cancelada termina em falha, com o erro `demanda cancelada pelo Auditor`.

## Regras

- A decisão só cabe na fase `aprovacao_demanda`, com a demanda já enviada. Fora disso, o gate informa que a decisão já foi tomada ou que a demanda ainda não foi enviada.
- O motivo da recusa tem de 1 a 2.000 caracteres.
- Título, tipo, descrição, critérios, desenho e custos chegam congelados do envio. O Auditor não os reescreve neste gate.
- Até o planejamento ser aprovado, o Auditor pode incluir ou remover dependências da demanda, com as mesmas regras do registro: sem ciclo e sem autorreferência.
- A IA externa e o agente de dentro não decidem este gate.

## Pós-condições

Aprovada, a demanda espera o planejamento e o agendador pode pegar o ticket quando as dependências estiverem concluídas. Recusada, a demanda e as dependentes ficam canceladas, com o motivo, e saem do andamento.
