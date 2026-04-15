# Teoria dos Grafos e Computabilidade - Aula 14

## Problema proposto

Encontre o tamanho dos menores caminhos de um vértice "a" para todos os outros vértices.

### Resolução

A ideia por trás desse problema é a realização do algoritmo de Dijkstra, que utiliza o relaxamento
das arestas para identificar o menor caminho.

Desse modo, tem-se uma implementação em pseudocódigo do algoritmo:

```
Dijkstra(grafo, a)

        para todo v e V
            cor[v] = branco
            d[v] = infinito
        fim para

        cor[a] = cinza
        d[a] = 0
        fila.insere(a)

        enquanto !fila.vazio()
            u = fila.remove_prioridade_min()
            para todo u* e N(u)
                se cor[u*] = branco e d[u*] > d[u] + peso(u, u*)
                    fila.insere(u*)
                    cor[u*] = cinza
                    d[u*] = d[u] + peso(u, u*)
                fim se
            fim para
        fim enquanto
fim algoritmo
```