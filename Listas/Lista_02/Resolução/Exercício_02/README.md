# Teoria dos Grafos e Computabilidade - Lista 02

## Exercício 02

O professor Strong afirma que o algoritmo para componentes fortemente conexos
pode ser simplificado usando o grafo original (em vez do transposto) 
na segunda busca em profundidade graças ao caminhamentos nos vértices em ordem 
crescente de tempos de término. O professor está correto?

### Resolução

O professor Strong está errado, pense no seguinte exemplo: Seja G = (V, E) um
grafo direcionado em que V possui os vértices 1, 2 e 3, e E possui as arestas
(2, 3) e (2, 1) e (3, 1).

Tendo este exemplo, ao realizar o DFS iniciando no vértice dois e indo para o três, é
possível perceber que a ordem de término será: 3, 1, 2. Ao realizar sobre o mesmo grafo
um DFS utilizando a ordem acima, o vértice 3 alcançará o vértice 1, o que iria demarcá-lo
como parte do componente fortemente conexo mesmo não sendo. Tem-se, portanto, a prova 
por contradição.

