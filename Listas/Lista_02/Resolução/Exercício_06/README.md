# Teoria dos Grafos e Computabilidade - Lista 02

## Exercícios 06

Seja G = (V, E) um grafo direcionado. Identifique, justificando sua resposta, o menor número de arestas que
devem ser inseridas para transformar o grafo em fortemente conexo.

### Resolução

Para o grafo se tornar fortemente conexo é preciso que para todo par de vértices exista um caminho de ida
e de volta. Desse modo, o número de arestas mínimo para ele se tornar fortemente conexo é o max entre
o número de bases e anti-bases.

A logica por trás disso está em transformar os componentes fortemente conexos em um vértice de modo que
o grafo seja um acíclico (DAG), a partir disso basta analisar os graus dos vértices para criar os caminhos
de ida e de volta, pois, ao fazer um ciclo com as bases ou anti-bases, o grafo de componentes fortemente conexos
será um ciclo, garantindo a ida e a volta.