# UC-CFG-02 — Cadastrar um projeto

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Configuração → Novo projeto |

## Objetivo

O Auditor registra um repositório para a Fabriquinha executar o SDLC nele. Ele escolhe as linguagens do projeto e, para cada uma, o caminho do código, o comando de teste e, quando houver, os testes e2e.

## Pré-condições

O Auditor está no web. O harness de agentes está disponível para listar modelos. O CLI da hospedagem que o remoto usar está autenticado na máquina do serviço.

## Fluxo principal

1. O Auditor abre **+ Novo projeto**.
2. Preenche a identificação, a branch base, a chave do SonarQube e, se quiser, os comandos de preparação.
3. Inclui uma ou mais linguagens. Em cada uma escolhe a linguagem, informa o caminho absoluto do código e o comando que roda os testes dessa linguagem.
   Se a linguagem tem testes e2e, preenche o bloco e2e dela.
4. Se quiser, inclui fitness functions. Pode deixar esta lista vazia e completá-la depois, na edição.
5. Para cada etapa executada pelo agente, pode escolher o modelo e, ao lado, o effort. Os dois podem ficar no padrão do harness.
6. Ao cadastrar, a Fabriquinha confere o repositório descoberto e cada modelo escolhido.
7. Com a conferência aceita, grava o projeto, com as linguagens e o comando de cada uma.
8. Em seguida varre a branch base e fixa o cursor dos commits humanos. Falha nessa varredura fica no log; o cadastro permanece. A varredura do dia seguinte refaz.

## Dados

### Identificação

| Campo | Regra |
|---|---|
| Nome | Slug único: letra ou número, depois letras minúsculas, números e hífen, até 64 caracteres. É a chave do projeto e não muda depois. |
| Branch base | Obrigatória. A Fabriquinha confere que ela existe no remoto `origin`. |
| SonarQube | Chave do projeto, obrigatória. A evidência de análise estática da implementação sai deste projeto. O token fica no ambiente do serviço, fora do banco e fora deste formulário. |
| Preparação | Lista opcional de comandos de shell. A Fabriquinha os roda ao criar a worktree da rodada, para o agente e os testes encontrarem as dependências. Falha num comando falha a execução, com o log. |

O Auditor não informa um caminho único de repositório nem uma lista única de testes. A raiz do git, a hospedagem e os testes saem das linguagens.

### Linguagens

Pelo menos uma. Cada item:

| Campo | Regra |
|---|---|
| Linguagem | Escolhida no catálogo: Rust, TypeScript, JavaScript, Java, Kotlin, Python, Go, C#, Ruby, PHP, Swift, C, C++. A mesma linguagem pode aparecer mais de uma vez, com caminhos diferentes. |
| Caminho | Absoluto e existente, apontando para o diretório daquela linguagem. |
| Comando de teste | Obrigatório. A Fabriquinha o executa nesse caminho. Na implementação, a evidência de testes exige código 0 em todos os comandos de todas as linguagens. Não roda os testes e2e. |

Todos os caminhos pertencem ao mesmo repositório git. A Fabriquinha descobre a raiz com a raiz que o git declara para cada caminho e grava essa raiz no projeto. A hospedagem fica só leitura, deduzida da URL do remoto `origin`: GitHub ou Azure DevOps.

### Testes e2e

Opcional em cada linguagem.
Quando existe, separa os testes e2e dos testes gerais: o comando de teste da linguagem não os roda.
Os comandos rodam no caminho da linguagem.

| Campo | Regra |
|---|---|
| Arquivos | Um ou mais globs, relativos à raiz do git. Todo arquivo que casa com um deles é arquivo e2e. É por eles que o planejamento separa a rodada de e2e (UC-PLAN-01). |
| Comando de build | Obrigatório. Compila ou coleta os testes e2e sem executá-los. Erro de compilação ou de coleta sai com código diferente de 0. |
| Comando de execução | Obrigatório. Executa os testes e2e. |
| Código de falha | Obrigatório, inteiro de 1 a 255. É o código com que o comando de execução sai quando um teste falha, sem erro do runner. Na rodada de e2e, a execução que sai com ele é aceita (UC-IMPL-01). |

Ao escolher uma destas linguagens, o bloco já vem preenchido, menos os arquivos.
O Auditor pode trocar qualquer valor.

| Linguagem | Build | Execução | Código de falha |
|---|---|---|---|
| Rust | `cargo test --no-run` | `cargo test` | 101 |
| Python | `python -m pytest --collect-only -q` | `python -m pytest` | 1 |
| JavaScript e TypeScript, via Node (React e Angular) | `npx playwright test --list` | `npx playwright test` | 1 |

### Fitness functions

Opcionais no cadastro. Cada uma, quando existe:

| Campo | Regra |
|---|---|
| Nome | Slug único dentro do projeto. |
| Comando | Obrigatório. |
| Descrição | Obrigatória, até 300 caracteres. |

Com pelo menos uma fitness function, a implementação e a revisão da rodada rodam os comandos e montam a matriz. Com a lista vazia, essas etapas seguem sem matriz de fitness.

### Modelo e effort por etapa

Um seletor de modelo por etapa, alimentado pelos modelos que o harness lista. Ao lado de cada seletor, o effort daquela etapa.

| Campo | Regra |
|---|---|
| Modelo | Opcional. Vazio usa o modelo padrão do harness. Preenchido, o par provedor + modelo precisa existir na lista do harness. A Fabriquinha confere antes de gravar. |
| Effort | Opcional, ao lado do modelo. Vazio usa o effort padrão do harness. Preenchido, um destes: `off`, `minimal`, `low`, `medium`, `high`, `xhigh`, `max`. A Fabriquinha passa o valor na sessão daquela etapa. Pode ser informado mesmo quando o modelo fica no padrão. |

Etapas com modelo e effort:

- planejar trabalho
- implementar
- revisar a rodada
- encerrar a rodada
- sessão em aberto
- revisar a demanda
- resolver conflito de merge

O desenho da necessidade acontece fora da Fabriquinha e não tem modelo aqui. O merge bem-sucedido do encerramento da demanda a Fabriquinha faz sozinha: a branch `fabriquinha/I-n`, já com os pull requests das rodadas, entra na branch base. O effort de **resolver conflito de merge** vale quando esse merge falha e abre sessão.

## Conferência antes de gravar

A falha interrompe o cadastro e nada é gravado.

1. Cada caminho existe e é diretório.
2. Todos os caminhos declaram a mesma raiz de repositório git.
3. O git dessa raiz tem `user.name` e `user.email`.
4. Existe remoto `origin` e a URL é GitHub ou Azure DevOps, nos formatos abaixo.
5. A branch base existe nesse remoto.
6. O CLI da hospedagem está autenticado: `gh auth status` no GitHub; `az account show` com a extensão `azure-devops` no Azure DevOps.
7. Cada modelo informado existe no harness.

| Hospedagem | URLs de `origin` |
|---|---|
| GitHub | `https://github.com/<dono>/<repo>[.git]`, `git@github.com:<dono>/<repo>[.git]` |
| Azure DevOps | `https://dev.azure.com/<org>/<projeto>/_git/<repo>`, `https://<org>.visualstudio.com/<projeto>/_git/<repo>`, `git@ssh.dev.azure.com:v3/<org>/<projeto>/<repo>`, `<org>@vs-ssh.visualstudio.com:v3/<org>/<projeto>/<repo>` |

## Fluxos alternativos

- **Campo inválido.** Erro ao lado do campo. O rodapé diz quantos campos têm erro.
- **Repositório, branch, remoto ou autenticação recusados.** A mensagem diz o que corrigir e aparece no rodapé do formulário.
- **Modelo recusado.** Erro no seletor da etapa.
- **Varredura da base falha depois da gravação.** O projeto permanece cadastrado. A falha vai para o log.

## Fora deste caso

Testes de estresse e análise de segurança ficam com o agente, na revisão da demanda. A configuração do projeto não tem usuários simulados, tarefas de simulação nem link de produto.

## Pós-condições

O projeto existe, com as linguagens, o comando de teste e o bloco e2e de cada uma, a raiz do git descoberta, a hospedagem e a branch base. O agendador pode passar a considerar o trabalho desse projeto quando estiver ligado.
