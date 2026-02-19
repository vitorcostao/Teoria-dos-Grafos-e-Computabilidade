# Teoria dos Grafos e Computabilidade - Aula 03

## Reflexão: Problema proposto

Projetar uma solução para identificar o número de componentes conexos de um grafo não direcionado.

### Resolução

Para resolver o problema é preciso entender os algoritmos de busca em grafos, como **DFS (Busca em Profundidade)** e **BFS (Busca em Largura)**, que permitem explorar todos os vértices conectados a um vértice inicial.

O processo consiste em:

1) Marcar todos os vértices do grafo como não visitados.

2) Inicializar um contador de componentes conexos com zero.

3) Percorrer todos os vértices do grafo:

   - Se um vértice ainda não foi visitado:

     - Incrementar o contador de componentes conexos.

     - Executar uma busca a partir desse vértice, visitando todos os vértices alcançáveis.

4) Repetir o processo até que todos os vértices tenham sido visitados.

Dessa forma, cada vez que se inicia uma nova busca a partir de um vértice não visitado, identifica-se um novo componente conexo. 
O algoritmo termina quando todos os vértices já foram explorados, e o contador final representa o número total de componentes conexos do grafo.

### Observações

- Vértices isolados (sem arestas) também são considerados componentes conexos individuais.

- Um grafo é considerado conexo se possuir apenas um componente conexo.

- A escolha entre DFS ou BFS não altera o número de componentes, apenas a ordem na qual os vértices são visitados.