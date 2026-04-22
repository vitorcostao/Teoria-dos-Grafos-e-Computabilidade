# Teoria dos Grafos e Computabilidade - Aula 17

## Exercício Proposto

Seja G = (V, E) um grafo não-direcionado e conexo, e G = (G, w) um grafo ponderado em que w:E -> Reais positivos.
Altere o Dijkstra para encontrar o subgrafo que seja conexo cuja a soma de pesos seja a menor possível. O subgrafo
deve ter a cardinalidade de V.

### Resolução

Para solucionar o problema, basta realizar algum algoritmo das árvores geradoras mínimas. Sendo eles, Kruskal, Prim ou 
Reverse-Delete.