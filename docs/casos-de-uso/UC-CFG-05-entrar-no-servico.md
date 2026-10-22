# UC-CFG-05 — Entrar no serviço

| | |
|---|---|
| Ator | Auditor |
| Superfície | CLI, `fabriquinha servidor` e `fabriquinha abrir` |

## Objetivo

O Auditor sobe o serviço e abre o web no navegador.

## Pré-condições

`fabriquinha servidor` e `fabriquinha abrir` rodam sem `FABRIQUINHA_EXECUCAO`.
Com essa variável, os dois saem com erro: "<comando> não é comando do agente (o serviço já está rodando; siga com o seu ticket)".
A raiz é `$FABRIQUINHA_ROOT`, ou `~/fabriquinha`.

## Fluxo principal

1. O Auditor roda `fabriquinha servidor`.
2. O serviço ocupa `127.0.0.1:<porta>` antes de abrir o banco.
3. Conecta ao PostgreSQL da Fabriquinha pela URL de `$FABRIQUINHA_BANCO` e aplica as migrações pendentes. A execução que ficou no ar segue UC-EXE-02.
4. Grava `<raiz>/web.url` e escreve no stdout `{"porta":<porta>}`.
5. O Auditor roda `fabriquinha abrir`. O comando lê `web.url`, abre esse endereço no navegador e escreve no stdout `{"url":"<url>"}`.
6. O navegador abre `/home`.

O sistema operacional só mantém o processo de pé.
O recomendado é um serviço de usuário do systemd.
A configuração do agendador é a de UC-CFG-01.

## Regras

| Regra | O que vale |
|---|---|
| Porta | Escuta só em `127.0.0.1:$FABRIQUINHA_PORTA` (padrão 4317). A porta é a trava de instância única. A segunda instância não a abre, sai com erro e não abre o banco. A mensagem é "a Fabriquinha não conseguiu abrir 127.0.0.1:<porta>: … (se outra Fabriquinha já está rodando, use-a; senão, libere a porta ou troque FABRIQUINHA_PORTA)". |
| Banco | `$FABRIQUINHA_BANCO` é obrigatória, no formato `postgres://<usuário>:<senha>@<host>:<porta>/<banco>`. O PostgreSQL roda numa imagem Docker; o serviço roda fora do Docker. Sem a variável, ou sem conexão, o serviço sai com erro, libera a porta e não grava `web.url`. A mensagem é "a Fabriquinha não conseguiu conectar ao PostgreSQL em <host>:<porta>: … (suba o contêiner do banco ou corrija FABRIQUINHA_BANCO)". A senha não aparece na mensagem. |
| Acesso | Sem login, sem token e sem cookie. O serviço só escuta em `127.0.0.1` e não fica exposto na rede. |
| `web.url` | Recebe `http://127.0.0.1:<porta>/home`. Ao parar, o serviço apaga o arquivo. |
| `abrir` | Sem o arquivo, sai com "o serviço não está rodando: falta <raiz>/web.url (suba-o com fabriquinha servidor)". Se o navegador não abre, a mensagem traz a URL para o Auditor abrir. |
| Host | `Host` diferente de `127.0.0.1:<porta>` recebe 403 em qualquer caminho. O corpo é `{ "erro": "só 127.0.0.1:<porta> é atendido" }`. |
| Origem | Pedido que não é `GET`, com `Origin` presente e diferente de `http://127.0.0.1:<porta>`, recebe 403. O corpo é `{ "erro": "pedido de outra origem recusado" }`. O CLI não manda `Origin` e passa. |
| Caminhos | `/api/*` atende o web e o CLI. `/static/*` entrega os arquivos do web. Os outros `GET` devolvem o `index.html`. |

## Pós-condições

Um processo escuta em `127.0.0.1:<porta>`.
`<raiz>/web.url` guarda o endereço do web.
O web atende o Auditor em `/home`.
