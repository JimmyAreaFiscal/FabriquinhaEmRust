# Restrições a seguir

1. Sempre que o usuário transmitir uma tarefa e pedir que ele valide antes de continuar, você não poderá seguir a tarefa antes que ele valide. 

2. O foco é escrever um código que seja fácil para leitura por um humano. Para isso, deve-se:

a) Usar nomes que expliquem os dados
```
// BAD
let em: Glob_e2e = ...

// GOOD
let caminho_e2e_do_desenho: Glob_e2e = ...
```

b) Separar funções de forma que cada função represente uma ação para o humano

c) Dar nomes mais precisos às funções auxiliares 

