# UC-DEM-01 — Registrar a demanda

| | |
|---|---|
| Ator | IA externa (pi, Claude Code ou Codex), com o Auditor |
| Superfície | CLI `demanda` |

## Objetivo

A IA externa entrega uma demanda pronta para o gate do Auditor. O entendimento da necessidade e o desenho acontecem fora da Fabriquinha. O CLI só recebe o resultado.

## Pré-condições

- O projeto já está cadastrado (UC-CFG-02).
- O comando roda sem execução da Fabriquinha aberta. O ambiente traz a sessão do harness: `PI_SESSION_ID`, `CLAUDE_CODE_SESSION_ID` ou `CODEX_THREAD_ID`. O CLI monta o ator externo com a ferramenta e essa sessão.
- Fora da Fabriquinha, a IA e o Auditor já produziram os critérios de aceite e o desenho da solução, e mediram os tokens e o custo da etapa.

## Fluxo principal

1. A IA registra a demanda: projeto, título, tipo (`funcionalidade` ou `problema`), descrição e, se houver, as demandas de que esta depende.
2. A Fabriquinha cria a demanda na fase `demanda`, com identificador `I-n`, presa à ferramenta e à sessão que a registrou. O ticket de demanda nasce em execução, sem processo da Fabriquinha: a IA trabalha nele até o envio.
3. A IA consulta o schema de cada tipo (`demanda schema <tipo>`) e submete as evidências, com `--dados` em JSON e os arquivos ao lado do comando.
4. Submeter de novo o mesmo tipo substitui a evidência anterior.
5. Com as evidências obrigatórias aceitas, a IA envia a demanda à aprovação.
6. A Fabriquinha marca o envio, congela título, tipo, descrição e custos, e leva a demanda à fase `aprovacao_demanda`.

## Evidências obrigatórias para enviar

| Tipo | Conteúdo |
|---|---|
| `criterios_aceite` | Um ou mais critérios. Cada um tem `id` `CA-n`, `dado`, `quando` e `entao`. O `id` não se repete. Sem arquivo anexo. |
| `desenho_solucao` | `resumo` de até 2.000 caracteres, para as etapas seguintes. `arquivosPrevistos`: ao menos um caminho a partir da raiz do repositório, com glob, ação `criar`, `alterar` ou `remover`, e motivo. `foraDeEscopo`: lista do que o desenho não vai fazer. Um HTML autocontido, com o desenho e os diagramas. |
| `custos_realizados` | Uma ou mais sessões do harness, cada uma com ferramenta (`pi`, `claude-code` ou `codex`), identificador da sessão e as linhas de uso: provedor, modelo, tokens de entrada, saída, cache de leitura e cache de escrita. O pi pode trazer o custo pronto. Sem arquivo anexo. |

O desenho da solução é a visão do que será implementado. O planejamento, na etapa seguinte, reparte esses arquivos e esses critérios em rodadas.

## Regras da demanda

| Campo | Regra |
|---|---|
| Título | Uma linha, de 1 a 120 caracteres. |
| Descrição | Markdown de até 1.000 palavras. Vem de arquivo, ou de stdin com `-`. |
| Tipo | `funcionalidade` ou `problema`. |
| Dependências | Outras demandas, de qualquer projeto. Sem ciclo e sem depender de si mesma. Informar de novo a mesma dependência não duplica. |
| Sessão | Só a ferramenta e a sessão que registraram a demanda leem, editam, anexam evidência e enviam. Outra sessão é recusada. |
| Edição | Até o envio, a mesma sessão altera título, tipo, descrição e dependências. |
| Arquivos | Saem do diretório em que o comando rodou. HTML, imagem e vídeo seguem o formato de evidência: HTML autocontido até 5 MiB; o desenho é um único HTML. |
| Custos | Linha sem custo pronto precisa ter preço conhecido para o par provedor + modelo. Sessão já submetida em outra demanda fica com as linhas zeradas e a referência à primeira. Depois do envio, o retrato não muda. |
| Envio incompleto | Falta uma evidência obrigatória, ou o conteúdo é inválido: a demanda permanece na fase `demanda` e a mensagem diz o que corrigir. |

## Comandos

| Comando | Efeito |
|---|---|
| `demanda registrar --projeto <p> --titulo <t> --tipo <funcionalidade\|problema> [--descricao <arquivo\|->] [--deps <I-1,I-2>]` | Cria a demanda. |
| `demanda editar I-n [--titulo …] [--tipo …] [--descricao …] [--deps …]` | Altera a demanda ainda não enviada. |
| `demanda schema <tipo>` | Devolve o JSON Schema do tipo. |
| `demanda evidencia I-n <tipo> [--dados <arquivo.json\|->] [--arquivo <caminho>]...` | Grava ou substitui a evidência. |
| `demanda enviar I-n` | Envia ao gate do Auditor. |
| `demanda ver I-n` | Lê a demanda da mesma sessão, até o envio. |

A saída de sucesso é um JSON em uma linha. Erro de regra ou de escopo sai no stderr com código 1. Uso inválido sai com código 2 e a ajuda do grupo `demanda`.

Sem sessão de harness no ambiente, o CLI recusa o grupo `demanda` e pede para rodar pela skill de registro no harness. Com uma execução da Fabriquinha aberta, estes comandos ficam indisponíveis: o agente de dentro não registra demanda.

## Fora deste caso

O comportamento atual, o protótipo, o relatório de complexidade e o relatório de impactos na arquitetura não entram no envio. O quadro SDLC AI-BASED pede, nesta etapa, critérios de aceite, desenho da solução, e a métrica de tokens e custo. Testes de estresse e análise de segurança ficam com o agente, na revisão da demanda.

## Pós-condições

A demanda está em `aprovacao_demanda`, com as três evidências gravadas. O Auditor decide no web (UC-DEM-02). A IA externa não altera mais essa demanda.
