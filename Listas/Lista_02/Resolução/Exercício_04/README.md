# Teoria dos Grafos e Computabilidade - Lista 02

## Exercício 04

Dado um grafo direcionado G = (V, E), explique como criar outro grafo G′ = (V, E′) tal que (a) G′
tenha os mesmos componentes fortemente conexos que G , (b) G′ tem o mesmo grafo de componentes que G,
e (c) E′ seja o menor possível. Descreva um algoritmo (rápido) para encontrar G′.

### Resolução

Para construir o grafo G′, basta encontrar os componentes fortemente conexos do grafo original e preservar
as arestas que conectam esses componentes, isso faz com que grafo de componentes de G′ seja igual de G. Além
disso, para manter o número mínimo de arestas, é preciso que, ao encontrar os componentes fortemente 
conexos do grafo original, os vértices desses componentes formem um ciclo, pois o ciclo garante a propriedade
de conectividade e mantém o número mínimo de arestas. 