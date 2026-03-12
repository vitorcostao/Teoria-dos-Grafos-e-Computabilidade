# Teoria dos Grafos e Computabilidade - Aula 05

## Problema proposto

Seja $G = (V, E)$ um grafo direcionado. Seja $v \in V$ um vértice qualquer. Projete uma solução para encontrar todos os vértices que são alcançados por $v$. 
E como ficaria para encontrar os vértices que alcançavam $v$.

## Resolução

Para saber quais vértices podem ser alcançados a partir de um vértice qualquer é preciso usar o algoritmo de busca em largura (BFS). 

### 1) Estrutura básica

O BFS geralmente utiliza uma fila (queue) para controlar a ordem de visitação dos vértices:

- Vértices visitados são marcados para não serem visitados novamente.
- Vértices são enfileirados quando encontrados, mas ainda não visitados.

### 2) Passos do algoritmo

```
todos_alcancados(grafo, v)
   para todo v* e V
      cor[v*] = branco
   fim para

   alcancados = []
   se existe aresta de v para v
      alcancados.insere(v)
   fim se

   f.insere(v)
   cor[v] = cinza
   
   enquanto !f.vazio()
      u = f.remove()
      para todos u* e N[u]
         se cor[u*] = branco
            f.insere(u*)
            alcancado.insere(u*)
            cor[u*] = cinza
         fim se
      fim para
      cor[u] = preto
   fim enquanto
   retorna alcancado
fim algoritmo



```

### 3) Vértices que alcançavam $v$

Para determinar os vértices que alcançavam $v$ basta inverter as arestas do grafo (Grafo Transposto) e aplicar o BFS.

---

## Novos conceitos

#### Fecho transitivo direto

- O fecho transitivo direto de um vértice 𝑣 é o conjunto de todos os vértices que podem ser alcançados a partir de 𝑣.

#### Fecho transitivo inverso

- O fecho transitivo inverso de 𝑣 é o conjunto de todos os vértices que podem chegar a 𝑣.