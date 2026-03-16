# Teoria dos Grafos e Computabilidade - Aula 12

## Problema proposto

Seja G = (v, e) um grafo não direcionado. Projete uma solução para encontrar um vértice, 
cuja remoção "desconecta" o grafo. Projete uma solução para encontrar uma aresta, cuja remoção
"desconecta" o grafo. E se fossem conjunto de vértices e arestas.

### Resolução

Para ambos os caso, basta testar cada vértice ou aresta e verificar se sua remoção aumenta o número
de componentes conexos do grafo. Desse modo, surge dois conceitos: Ponto de articulação (Vértice) e
Ponte (Arestas).

>OBS: Quanto mais pontos de articulação e pontes, mais robusto o grafo é, todavia isso possui custos.

Para conjuntos de vértices e arestas, basta verificar o conjunto potência e encontrar aqueles de menor
grau.

---

## Problema para reflexão

Como caminhar no grafo passando por todos os vértices uma única vez?

### Resolução

Primeiro é necessário identificar as ocasiões em que tais coisas são possíveis. Desse modo, em um
grafo euleriano (Todos os que vértices possuem grau par) e em um grafo semi-euleriano (Possui dois
vértices de grau impar e o resto de grau par) é possível realizar esse caminhamento. Todavia, em 
um grafo que possui quatro ou mais vértices de grau impar isso não é possível.

1) Verifique se o grafo é válido (0 ou 2 vértices ímpares).
2) Comece em um vértice de grau ímpar (se houver) ou em qualquer vértice (se todos forem pares).
3) Percorra as arestas, evitando atravessar pontes, a menos que não haja alternativa.
4) Exclua as arestas já percorridas.