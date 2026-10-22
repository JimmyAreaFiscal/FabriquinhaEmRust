# UC-PRJ-02 — Ver o heatmap

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Projetos → aba Heatmap |

## Objetivo

O Auditor vê onde o código da branch base mais mudou nos últimos 12 meses.

## Pré-condições

O projeto está cadastrado em UC-CFG-02.
A branch base é a branch desse cadastro, conferida no remoto `origin`.

## Fluxo principal

1. O Auditor abre a aba **Heatmap** na página do projeto (UC-PRJ-01).
2. Enquanto a leitura corre, a página mostra "Lendo o git…".
3. O seletor de projeto permanece disponível.
4. A Fabriquinha lê `refs/remotes/origin/<branch base>` no repositório do projeto.
5. A página mostra o início e o fim da janela, a branch base, o total de alterações e a árvore de pastas e arquivos.
6. Abrir uma pasta mostra os filhos dela.
7. Abrir um arquivo mostra a métrica e os commits da janela que tocaram esse arquivo.
8. Trocar o projeto no seletor abre o heatmap desse projeto.

## Regras

A leitura não busca no remoto.
Cada abertura da aba relê o git.
Abrir uma pasta ou um arquivo usa a leitura já feita.
A aba não se atualiza sozinha.

A janela termina no instante da leitura.
O início recua 12 meses no calendário UTC.
As datas da janela e dos commits aparecem como dd/mm/aaaa, no fuso do navegador.

| Peça | Regra |
|---|---|
| Arquivo na ponta | Entra na árvore, também quando não teve commit na janela. |
| Arquivo que a ponta já não tem | Fica de fora. |
| Renomeação | Não é seguida. |
| Nome contado | O nome do arquivo na ponta da branch base. |
| Alteração | Um commit que não é merge e que toca o arquivo dentro da janela. |
| Pasta | Soma as alterações dos arquivos abaixo dela, em qualquer nível. |
| Mesmo commit em dois arquivos | Conta duas vezes na pasta e no total do projeto. |
| Ordem dos filhos | Maior total primeiro. Empate pelo nome. |
| Total do projeto | Soma das alterações de todos os arquivos da ponta. |
| Total 0 | A página mostra "Nenhuma alteração nos últimos 12 meses". |
| Árvore com total 0 | Permanece, com os totais em 0. |

O calor da linha é ln(1+n) / ln(1+T).
n é o total da linha.
T é o total do projeto.
Com o total do projeto em 0, o calor de cada linha é 0.
A cor vai do azul, frio, ao vermelho, quente, conforme o calor.
A página nomeia os extremos como frio e quente.

O arquivo aberto mostra "Código alterado: N alterações · quanto maior, pior".
Com uma alteração, o texto usa "alteração".
A tabela lista o hash com 7 caracteres, o assunto, a data e o PR.
A lista vai do commit mais novo ao mais antigo.
Sem alteração, a tabela não aparece.
PR presente aparece como "PR #N".
PR ausente aparece como "sem PR".

O número do PR sai só do texto do git.
A atribuição percorre a linha first-parent da branch base, do merge mais antigo ao mais novo.

| Commit | PR |
|---|---|
| Merge cujo assunto começa com `Merge pull request #N` ou `Merged PR N` | N, nos commits que esse merge trouxe pelo segundo pai e que nenhum merge anterior da linha trouxe. |
| Commit dessa linha, que não é merge, com `(#N)` no fim do assunto | Squash, com o PR N. |
| O restante dos commits contados | sem PR. |

## Fluxos de erro

No erro de leitura, o seletor de projeto permanece.

- **Repositório ausente no disco.** A página mostra "Não foi possível montar o heatmap." A mensagem é `o repositório <caminho> não existe mais no disco (corrija o caminho do repositório do projeto na Configuração)`.
- **Branch base ausente.** A página mostra o mesmo título. A mensagem é `a branch base <nome> não existe em <caminho> (busque-a com git fetch origin <nome> ou corrija a branch base na Configuração)`.
- **Outra falha do git nessa leitura.** O mesmo título. A mensagem traz o comando que falhou e, entre parênteses, "corrija o repositório e execute o ticket de novo".
- **Projeto fora da lista.** A página mostra "Projeto não encontrado", o mesmo aviso de UC-PRJ-01. O texto cita o nome. O texto diz que o projeto não está cadastrado. **Ver os projetos** volta a `/projects`.

## Pós-condições

Na leitura aceita, o Auditor vê o heatmap desta abertura.
Na falha, vê o motivo.
O repositório e o projeto cadastrado seguem como estavam.
