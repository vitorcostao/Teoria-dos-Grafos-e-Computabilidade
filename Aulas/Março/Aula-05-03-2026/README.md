# Teoria dos Grafos e Computabilidade - Aula 09 

## Problema proposto

Seja $$G = (V, E)$$ um grafo não direcionado. Projete uma solução para encontrar, para um vértice $$u$$ dado,
as distâncias para todos os vértices $v \in V$.

## Resolução

Para determinar a distância a partir de um vértice qualquer, deve ser feito um algoritmo utilizando a busca
em largura, pois através dos níveis é possível saber a distância dos vértices $$v$$ para um vértice $$u$$.

Lembrando que a distância, em grafos, representa o menor número de arestas de um caminho.

Para a solução, será implementado um pseudocódigo utilizando $$d$$ como sendo a distância para outros vértices,
e $$L$$ para ser um label.

---

```
algoritmo {

    para cada v ∈ V (d[v] = +infinito, L[v] = não visitado)
    d[u] = 0
    fila.inserir(u)
    L[u] = em processo

    while !fila.vazio() {

          v.remover()
          L[v] = visitado
          para cada v* ∈ N(v)
               se L[v*] = não visitado então
                  d[v*] = min(d[v*], d[v] + 1)
                  L[v*] = em processo
                  fila.inserir(v*)
               senão
                  // Não faz nada
    }
}
```

---

A ideia por trás deste algoritmo é marcar como "em processo" ao entrar na fila e como "visitado" ao sair da fila, pois assim não será possível
inserir elementos que já existam na fila, o que não gerará redundância.