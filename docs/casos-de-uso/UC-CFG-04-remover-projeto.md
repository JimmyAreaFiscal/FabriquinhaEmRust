# UC-CFG-04 — Remover um projeto

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Configuração → projeto já cadastrado |

## Objetivo

O Auditor tira um projeto da Fabriquinha, junto com o processo, os custos e os arquivos que ela guardou para ele.

## Pré-condições

O projeto existe e não tem execução em andamento.

## Fluxo principal

1. No projeto salvo, o Auditor escolhe **Remover**.
2. A confirmação mostra o nome do projeto e quantas demandas ele tem, e avisa que demandas, custos e arquivos da Fabriquinha desse projeto serão apagados.
3. **Cancelar** fecha a confirmação e não muda nada.
4. **Remover projeto** apaga, no banco, o projeto e o que depende dele: demandas, rodadas, tickets, execuções, evidências, decisões e uso (custo).
5. No disco, apaga as pastas de execução, as sessões do agente, os arquivos de evidência que nenhum outro projeto usa e as worktrees da Fabriquinha daquele projeto.
6. A tela volta ao agendador. Se algum arquivo do disco não pôde ser apagado, mostra um aviso por falha. O banco não volta atrás.

## O que permanece

O repositório git original, as branches criadas pela Fabriquinha e o remoto.

## Fluxos alternativos

- **Execução em andamento nesse projeto.** A Fabriquinha recusa a remoção, fecha a confirmação e mostra o motivo. Nada é apagado.
- **Projeto inexistente.** A remoção não ocorre.

## Pós-condições

O projeto deixa de aparecer na Configuração e no restante do web. O agendador não escolhe mais trabalho dele.
