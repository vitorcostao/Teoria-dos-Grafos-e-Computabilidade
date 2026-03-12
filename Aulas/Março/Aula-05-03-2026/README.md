# Teoria dos Grafos e Computabilidade - Aula 09 

## Problema proposto

Seja $$G = (V, E)$$ um grafo não direcionado. Projete uma solução para encontrar, para um vértice $$u$$ dado,
as distâncias para todos os vértices $v \in V$.

## Resolução

Para determinar a distância a partir de um vértice qualquer, deve ser feito um algoritmo utilizando a busca
em largura, pois através dos níveis é possível saber a distância dos vértices $$v$$ para um vértice $$u$$.

Lembrando que a distância, em grafos, representa o menor número de arestas de um caminho.

---

```
distancias_vertice(grafo, v)
    para todo v* e V
        cor[v*] = branco
        d[v*] = infinito
    fim para

    d[v] = 0
    f.insere(v)
    cor[v] = cinza

    enquanto !f.vazio()
        u = f.remove()
        para todo u* e N[u]
            se cor[u*] = branco
                d[u*] = min(d[u*], d[u] + 1)
                cor[u*] = cinza
            fim se
        fim para
        cor[u] = preto
    fim enquanto
fim algoritmo
    
```

---

A ideia por trás deste algoritmo é marcar como "em processo" ao entrar na fila e como "visitado" ao sair da fila, pois assim não será possível
inserir elementos que já existam na fila, o que não gerará redundância.