---
name: refatorar-legibilidade
description: Analisa e refatora arquivos para facilitar a leitura por humanos, melhorando nomes e organização sem mudar comportamento. Use quando o usuário pedir revisão de legibilidade, clareza de código ou simplificação de código confuso. Exige validação humana antes de fazer commit e merge na branch principal.
---

# Refatorar para leitura humana

O objetivo é tornar a intenção do código evidente, não apenas reduzir linhas ou satisfazer ferramentas. Prefira mudanças pequenas e focadas.

## Regras obrigatórias

- Leia os `AGENTS.md` aplicáveis e respeite o estilo do projeto.
- Preserve comportamento, API pública, mensagens e prioridade dos erros, ordenação e representação dos dados. Mudanças nesses contratos exigem aprovação separada.
- Preserve alterações existentes, inclusive no arquivo-alvo. Não inclua trabalho alheio no commit.
- Não introduza arquitetura, dependências ou abstrações sem necessidade demonstrada.
- Se o usuário pedir somente análise, apresente o diagnóstico e aguarde autorização para refatorar.
- Se o usuário exigir validação em qualquer etapa, pare nessa etapa até receber aprovação explícita.
- **Após refatorar, sempre aguarde validação humana da legibilidade antes de fazer commit ou merge. Testes passando não substituem essa validação.**

## 1. Ler o arquivo e seu contexto

1. Confirme quais arquivos fazem parte da tarefa.
2. Leia o arquivo-alvo completo, incluindo testes relevantes.
3. Consulte apenas os tipos, consumidores e regras necessários para entender seus contratos.
4. Verifique `git status`, branch atual, diferenças locais e arquivos já staged, sem modificá-los.
5. Identifique os testes e comandos de validação existentes. Execute uma baseline quando viável e registre falhas anteriores à refatoração.

Se não entender uma regra, investigue ou pergunte; não a simplifique por suposição. Use exploração delegada quando disponível e exigida pelas instruções do ambiente.

## 2. Identificar dificuldades de leitura

Aponte problemas concretos, com arquivo, linha e exemplo. Procure:

- Nomes abreviados, genéricos ou dependentes de contexto distante, como `em`, `dados` ou `do_desenho`.
- Nomes que não revelam o conteúdo, a finalidade ou o efeito de uma função.
- Funções que misturam regras, transformação de dados e construção de mensagens.
- Aninhamento ou cadeias de operações que escondem a sequência das decisões.
- Helpers que apenas adicionam indireção, sem representar uma ação compreensível.
- Contratos, invariantes e motivos importantes que precisam ser deduzidos pelo leitor.

Diferencie dificuldade real de preferência pessoal. Não recomende mudanças apenas por métricas ou por um padrão abstrato.

## 3. Estudar uma solução de legibilidade

Antes de editar, explique brevemente o diagnóstico e o plano mínimo:

- Escolha nomes que expliquem os dados e distingam conceitos próximos, por exemplo `rodadas_por_criterio` e `caminhos_e2e_do_desenho`.
- Organize funções para que cada uma represente uma ação reconhecível para um humano.
- Deixe o fluxo principal revelar as etapas da operação.
- Extraia responsabilidades coesas; não pulverize o código em funções triviais nem crie wrappers que apenas repassem chamadas.
- Prefira o fluxo mais fácil de acompanhar, seja ele um laço ou uma cadeia de iteradores.
- Documente contratos e motivos, não traduza cada instrução em comentário.
- Liste os comportamentos delicados que precisam permanecer iguais e como serão verificados.

Se houver ambiguidade de escopo ou uma decisão que altere contratos, pergunte antes de implementar. Não transforme refatoração de leitura em otimização ou correção funcional silenciosa.

## 4. Refatorar e validar tecnicamente

1. Aplique apenas o plano autorizado, preservando as alterações locais.
2. Mantenha nomes, comentários e organização coerentes com o projeto.
3. Acrescente testes de caracterização quando houver lacunas relevantes nos contratos afetados, preferencialmente antes da mudança.
4. Execute os testes pertinentes, a verificação de formatação e os lints disponíveis. Em Rust, use as skills `rust-patterns` e `rust-testing` conforme necessário.
5. Formate somente os arquivos da tarefa; verificações mais amplas não devem reescrever arquivos alheios.
6. Revise o diff: confirme ausência de mudanças funcionais, alterações fora do escopo e abstrações desnecessárias.

Registre os comandos e seus resultados reais. Não diga que uma falha é preexistente sem evidência da baseline. Se a validação estiver bloqueada ou falhar, explique a limitação; não corrija arquivos alheios nem afirme sucesso completo.

## 5. Validar a legibilidade com o humano — parada obrigatória

Apresente:

- Arquivos alterados e melhorias principais, com exemplos curtos de antes/depois quando úteis.
- Contratos preservados e resultado das verificações técnicas.
- Falhas, limitações ou dúvidas restantes.

Pergunte explicitamente: **“A leitura ficou mais clara? Você aprova esta versão para commit e merge na branch principal?”**

**Pare e aguarde a resposta. Não faça stage, commit ou merge enquanto essa versão não for aprovada.**

Se o humano pedir ajustes, volte à refatoração, valide novamente e peça aprovação da nova versão. Se aprovar apenas a legibilidade, mas não autorizar commit e merge, peça essa autorização separadamente.

## 6. Fazer commit e merge na branch principal

Somente após a aprovação da versão e autorização para commit e merge:

1. Verifique novamente `git status`, branch, diff, staged e operações Git em andamento; o estado pode ter mudado durante a validação humana.
2. Confirme a branch principal pelas convenções do repositório ou configuração remota. Não suponha `master` ou `main`; se não estiver claro, pergunte.
3. Se houver merge, rebase, cherry-pick ou conflitos de outro trabalho em andamento, **pare e informe o bloqueio**. Não aborte nem resolva operações alheias. Aguarde sua conclusão ou autorização explícita para uma alternativa isolada.
4. Se houver alterações novas sobre a versão aprovada, revise-as e peça nova validação quando afetarem a refatoração.
5. Prepare o commit somente com as mudanças aprovadas e confira o diff staged. Use caminhos ou hunks específicos, nunca `git add .` indiscriminadamente. Se não puder separar alterações alheias com segurança, pergunte antes de continuar.
6. Faça um commit com mensagem objetiva, conforme o padrão do projeto, e confirme os arquivos incluídos.
7. Integre à branch principal respeitando o fluxo do repositório e sem descartar mudanças locais. Se o commit já estiver na branch principal, não crie um merge artificial: informe que a integração já ocorreu.
8. Em caso de conflito na integração, pare e apresente-o; não escolha automaticamente um lado nem amplie o escopo.
9. Valide a integração com os comandos pertinentes. Informe hash do commit, branch principal, resultado da integração e eventuais bloqueios.

Não use reset destrutivo, force push, descarte de alterações, stash automático ou reescrita de histórico para contornar dificuldades. Não faça push sem solicitação do usuário.
