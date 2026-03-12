# Teoria dos Grafos e Computabilidade - Aula 03

## Reflexão: Problema proposto

Projetar uma solução para identificar o número de componentes conexos de um grafo não direcionado.

### Resolução

#### Algoritmo usando a estrutura de dados pilha 
```
N_componentes_conexos(grafo)
   para todo v e V
      cor[v] = branco
   fim para

   ncc = 0

   para todo v e V
      se cor[v] = branco
         ncc += 1
         p.insere(v)
         cor[v] = cinza

         enquanto !p.vazia()
            u = p.remove()
            para todo u* e N[u]
               se cor[u*] = branco
                  p.insere(u*)
                  cor[u*] = cinza
               fim se
            fim para
            cor[u] = preto
         fim enquanto
      fim se
   retorna ncc
fim algoritmo
```

#### Algoritmo usando recursão
```
N_componentes_conexos(grafo)
   para todos v e V
      cor[v] = branco
   fim para

   ncc = 0

   para todo v e V
      se cor[v] = branco
         ncc += 1
         DFS_Visita(grafo, v)
      fim se
   fim para
   retorna ncc
fim algoritmo

DFS_Visita(grafo, u)
   cor[u] = cinza

   para todo u* e N[u]
      se cor[u*] = branco
         DFS_Visita(grafo, u*)
      fim se
   fim para
   cor[u] = preto
fim algoritmo
   
```

### Observações

- Vértices isolados (sem arestas) também são considerados componentes conexos individuais.

- Um grafo é considerado conexo se possuir apenas um componente conexo.

- A escolha entre DFS ou BFS não altera o número de componentes, apenas a ordem na qual os vértices são visitados.