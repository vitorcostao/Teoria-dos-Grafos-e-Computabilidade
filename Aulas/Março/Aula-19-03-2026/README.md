# Teoria dos Grafos e Computabilidade - Aula 13

## Problema proposto

Seja G = (V, E) um grafo não direcionado. Encontre um caminhamento que "passe" por todas arestas
voltando à origem. E se não for permitido repetir arestas? Como seria o caminhamento?

---

### Resolução 

Se for permitido a repetetição de arestas, basta realizar uma busca pelo algoritmo caso ele seja um
grafo conexo.

Todavia, em se tratando de um grafo que não pode possuir a repetição de arestas, é necessário que o 
grafo seja euleriano, ou seja, precisa-se ter um ciclo e todos vértices devem possuir um grau par.
Desse modo, tem-se o algoritmo de caminhamento:

```
escolhe_aresta(grafo, u) -> u*

    para todo u* em N(u)
        se (u, u*) não é ponte OU é a única aresta em u
            retorna u*
        fim se
    fim para
fim algoritmo

caminhamento(grafo, início) -> Retorna caminho

    se o grafo não for euleriano 
        retorna nulo
    fim se

    pilha_caminho = [início]
    u = início

    enquanto grafo.tem_arestas()
        u* = escolhe_aresta(grafo, u)
        pilha_caminho.insere(u*)
        grafo.remove_aresta(u, u*)
        u = u*
    fim enquanto
    retorna caminho
fim algoritmo

```

