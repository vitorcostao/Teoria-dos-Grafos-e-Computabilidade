# Teoria dos Grafos e Computabilidade - Lista 01

## Exercício 03

Prove que para qualquer grafo direcionado G, temos ((G^T)^SCC)^T = G^SCC . 
Ou seja, a transposta do grafo de componentes de GT é igual ao grafo de componentes de G.

### Resolução

O algoritmo de Kosaraju encontra os componentes fortemente conexos utilizando 
o conceito do grafo transposto para isolar tais componentes. Desse modo, 
o grafo transposto possui os mesmos componentes fortemente conexos do grafo
original. Assim, transpor um grafo de componentes transposto o torna no grafo
de componentes original.