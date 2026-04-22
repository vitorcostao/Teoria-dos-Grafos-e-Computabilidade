# Teoria dos Grafos e Computabilidade - Aula 18

## Algoritmo de Prim

É um algoritmo baseado em Dijkstra. Tal qual o passo a passo proposto pelo cientista holândes, Prim
também utiliza os menores pesos como prioridade a cada iteração, todavia, ao invés de realizar a soma
acumulada, o algoritmo apenas mantém o peso atual, o que produz uma árvore geradora mínima.

## Algoritmo de Kruskal

Tal proposta é dada da seguinte forma, o grafo inicia-se vazio, a partir disso ordena-se os pesos de modo crescente
e começa-se pela aresta de menor peso, se por acaso a aresta inserida no grafo vazio produzir um ciclo, a inserção
será ignorada passando para próxima aresta.

## Algoritmo Reverse-Delete

O grafo inicia-se como sendo igual ao original, ordena-se os pesos de modo decrescente e começa pelo maior. Caso a 
remoção deixe o grafo como sendo desconexo, tal remoção é ignorada.

