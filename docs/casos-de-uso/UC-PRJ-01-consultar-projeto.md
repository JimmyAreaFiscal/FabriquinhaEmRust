# UC-PRJ-01 — Consultar o projeto

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Projetos |

## Objetivo

O Auditor vê as demandas de um projeto, a fase de cada uma e quais estão fora do andamento.

## Pré-condições

O Auditor está no web. **Projetos** abre `/projects`.

## Fluxo principal

1. O Auditor abre **Projetos**.
2. Sem nome no caminho, a página escolhe o projeto e o caminho passa a `/projects/:projeto`.
3. A página mostra o seletor, as abas, os cards do mês, a contagem, as opções e a lista.
4. O Auditor troca o projeto no seletor. A página abre esse projeto.
5. O Auditor liga ou desliga **Mostrar concluídas**, **Mostrar arquivadas** e **Mostrar canceladas**. A lista redesenha.
6. O Auditor clica no título da demanda. Abre a página da demanda.
7. O Auditor clica na linha, fora de um link. A gaveta dos tickets abre ou fecha.
8. Com a aba visível, a página relê a cada 5 segundos, e também ao voltar à aba.

## O que a página mostra

O cabeçalho tem o título Projeto, o seletor e **Editar na Configuração** (UC-CFG-03). A demanda entra pelo CLI (UC-DEM-01).

A aba **Demandas** é esta página. A aba **Heatmap** abre o heatmap deste projeto (UC-PRJ-02).

Os cinco cards do mês corrente deste projeto estão em UC-MET-01.

A barra traz a contagem, as três opções e **Arquivar concluídas** (UC-DEM-04).

Com demandas na lista, o cabeçalho do trilho fica acima das linhas. A lista vazia mostra "Nenhuma demanda." e fica sem esse cabeçalho.

Cada linha traz o título, `I-n`, o tipo, o bloqueio, os pull requests mergeados sobre o total, o progresso dos tickets, o trilho e a sessão em destaque. Na fase `execucao`, o trilho ganha uma linha por rodada. A gaveta lista os tickets, agrupados pela rodada.

## Regras

### Projeto no caminho

| Situação | Página |
|---|---|
| `/projects`, com projetos | O último visto neste navegador, se ainda existe. Senão, o primeiro nome em ordem alfabética. O caminho vira `/projects/:projeto`. |
| Nenhum projeto | "Nenhum projeto cadastrado ainda." **Cadastrar um projeto na Configuração** abre o cadastro (UC-CFG-02). |
| Nome ausente do cadastro | "Projeto não encontrado." O texto cita o nome. **Ver os projetos** volta a `/projects`. |
| Navegador sem armazenamento | A página abre. O projeto visto desta vez não fica para a próxima. |

O seletor lista os projetos cadastrados. Trocar de projeto vai para `/projects/:projeto`.

### Contagem

A contagem reúne todas as demandas do projeto, com as opções ligadas ou desligadas.

| Número | Demanda |
|---|---|
| Em andamento | Fora de `concluida` e sem cancelamento. |
| Concluídas | Fase `concluida`, ainda por arquivar. |
| Arquivadas | Arquivada. |
| Canceladas | Cancelada. |

O texto usa o singular em 1: "1 concluída", "1 arquivada", "1 cancelada". Zero e os demais usam o plural. "Em andamento" fica igual.

### Quem entra na lista

As em andamento entram sempre. Cada opção inclui mais um grupo. As três começam desligadas. A escolha permanece no redesenho e na troca de projeto, até recarregar a página.

| Opção | Inclui |
|---|---|
| Mostrar concluídas | Concluída que não está arquivada nem cancelada. |
| Mostrar arquivadas | Arquivada que não está cancelada. |
| Mostrar canceladas | Cancelada. |

**Arquivar concluídas** fica desabilitado quando o número de concluídas é zero. O arquivamento é UC-DEM-04.

A lista segue do maior `I-n` para o menor.

### Linha

| Marca | Regra |
|---|---|
| Título | O título da demanda. Título vazio aparece como "sem título". O link abre a página da demanda. |
| `I-n` | O identificador da demanda. |
| Tipo | Aparece quando a demanda tem tipo. |
| Bloqueio | "bloqueada por I-n" para cada dependência ainda em andamento. |
| Pull request | Mergeados sobre o total das rodadas. Cada um abre o seu pull request. |
| Progresso | Tickets com status concluído, sobre o total de tickets. |
| Data | "cancelada em dd/mm" na cancelada. "arquivada em dd/mm" na arquivada que não está cancelada. "concluída em dd/mm" na concluída que não está arquivada. O dia é o do navegador. |

### Trilho

Cinco etapas. `aprovacao_demanda` e `aprovacao_planejamento` são gates. Na fase `execucao`, cada rodada tem o seu trilho: desenvolvimento, revisão, encerramento, aprovação do pull request e sessão em aberto. A aprovação do pull request é fase da rodada (UC-ENC-02).

| Etapa | Fases |
|---|---|
| Demanda | `demanda`, `aprovacao_demanda` |
| Planejar | `planejamento`, `aprovacao_planejamento` |
| Executar | `execucao` |
| Revisar | `revisao_demanda` |
| Encerrar | `encerramento_demanda` |

| Fase no trilho | Marca |
|---|---|
| Anterior à atual, e a demanda já esteve nela | Feita |
| A fase atual | Atual |
| Posterior, ou pulada | Sem marca |
| Demanda em `concluida` | Nenhuma fase atual |

O estado da linha sai das sessões ativas desta demanda. Falha vem na frente. Depois, sessão que aguarda o Auditor. Depois, sessão que roda.

Demanda cancelada usa o estado cancelada. A linha da cancelada e a da concluída ficam apagadas.

O cabeçalho das etapas e das fases permanece ao rolar a página e acompanha a rolagem horizontal do trilho.

### Sessão em destaque

À direita da linha fica uma sessão: a que falhou, senão a que aguarda, senão a que roda. O link abre o chat (UC-EXE-02).

| Sessão | Texto |
|---|---|
| Rodando, modo consulta | Consulta |
| Rodando, outro modo | Rodando |
| Aguardando, motivo falha | Falhou |
| Aguardando, motivo conversa, ou sem motivo | Conversa |
| Aguardando, motivo diálogo | Diálogo |
| Aguardando, gate do planejamento | Gate do planejamento |
| Aguardando, gate do PR | Gate do PR |

O texto traz há quanto tempo a sessão está assim.

### Gaveta

A gaveta abre sozinha quando a linha aguarda ou falhou. A que o Auditor abriu ou fechou permanece até recarregar a página. O botão da linha chama-se "Tickets de I-n".

| Coluna | Regra |
|---|---|
| Etapa | planejar, implementar, revisar a rodada, encerrar a rodada, sessão em aberto, revisar a demanda ou encerrar a demanda. |
| `T-n` | O identificador do ticket. |
| Título | O título do ticket. |
| Status | aberto, em execução, concluído, falhou, fechado ou cancelado. |
| Modelo | O do ticket, com ★. Na falta dele, o da etapa no projeto (UC-CFG-02). Na falta dos dois, o padrão da etapa. |
| Sessão | A sessão ativa daquele ticket, com o mesmo texto da linha. A célula fica vazia quando o ticket não tem sessão ativa. |

Ticket concluído sem sessão fica apagado na gaveta.

### Redesenho

O redesenho de 5 segundos espera enquanto o seletor está focado ou um diálogo está aberto. A rolagem horizontal do trilho fica onde o Auditor a deixou.

## Pós-condições

A consulta deixa demandas, tickets e a configuração como estavam. O navegador guarda o projeto visto quando o armazenamento existe.
