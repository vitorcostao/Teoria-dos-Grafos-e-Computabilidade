# Teoria dos Grafos e Computabilidade - Aula 15

## Problema proposto

Encontre o menor caminho entre os vértices "a" e "c".

### Resolução 

BellmanFord(grafo, s)

    para todo v ∈ V
        d[v] = infinito
    fim para

    d[s] = 0

    para i = 1 até |V| - 1
        para todo (u, v) ∈ E
            se d[v] > d[u] + peso(u, v)
                d[v] = d[u] + peso(u, v)
            fim se
        fim para
    fim para

fim algoritmo