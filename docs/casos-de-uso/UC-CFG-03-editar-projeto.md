# UC-CFG-03 — Editar um projeto

| | |
|---|---|
| Ator | Auditor |
| Superfície | Web, Configuração → projeto já cadastrado |

## Objetivo

O Auditor altera a configuração de um projeto já cadastrado, inclusive linguagens, comandos de teste, fitness functions e o effort de cada etapa.

## Pré-condições

O projeto existe. O Auditor abre o item dele no menu de Configuração.

## Fluxo principal

1. A tela mostra o mesmo formulário do cadastro (UC-CFG-02), preenchido com o que está gravado.
2. O nome aparece só leitura, com o número de demandas do projeto ao lado.
3. O Auditor altera o que precisar: branch base, SonarQube, preparação, linguagens (incluir, mudar caminho, comando ou bloco e2e, remover), fitness functions e modelo ou effort de cada etapa.
4. Fitness functions continuam opcionais. Uma lista vazia no cadastro pode receber itens nesta edição; itens já gravados podem ser editados ou removidos.
5. Ao salvar, a Fabriquinha repete a conferência do repositório descoberto e dos modelos escolhidos.
6. Com a conferência aceita, grava por cima. A edição não refaz a varredura da branch base nem move o cursor dos commits humanos.

## Regras

- O nome é a chave e permanece o do projeto aberto. Um nome enviado no corpo é ignorado.
- Linguagens, comandos, blocos e2e, fitness functions, modelos e efforts seguem as mesmas regras do cadastro.
- A raiz do git e a hospedagem são recalculadas a partir dos caminhos das linguagens e do remoto, e gravadas de novo.
- Erro de campo aparece ao lado do campo. Falha de repositório, branch, remoto ou autenticação aparece no rodapé. O que já estava gravado permanece até um salvamento aceito.
- **Cancelar** descarta o que foi digitado e volta a mostrar o projeto gravado.

## Pós-condições

O projeto fica com a configuração salva. Execuções já em andamento seguem com a configuração que tinham ao começar. A próxima execução usa a configuração nova.
