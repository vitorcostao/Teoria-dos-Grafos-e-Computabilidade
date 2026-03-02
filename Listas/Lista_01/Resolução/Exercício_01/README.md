# Teoria dos Grafos e Computabilidade - Lista 01

## Aula destinada à lista de exercícios 01 

### Exercício 01 (a)

Prove que o número de vértices de grau ímpar em um grafo deve ser par.

#### Resolução

De acordo com o Teorema do Aperto de Mão, a soma dos graus de todos os vértices deve ser igual ao dobro de arestas 
do grafo.

$$\sum_{v \in V} \deg(v) = 2|E|$$

Onde:  
- \(V\) = conjunto de vértices  
- \(E\) = conjunto de arestas  
- deg(v) = grau do vértice \(v\)  

Desse modo, é possível separar os somatório para vértices de grau ímpar e vértices de grau par. Nesse contexto, o somatório dos
graus pares será sempre par. Logo, para que o resultado dê $$2|E|$$, obrigatoriamente o somatório dos graus ímpares deve ser par.

---

### Exercício 01 (b)

Se 10 pessoas apertam as m˜aos umas das outras, quantos apertos de m˜ao ocorreram? O que essa quest˜ao
tem a ver com a teoria dos grafos?

#### Resolução

Usando o Teorema do Aperto de Mão, é possível identificar quantos apertos de mão ocorreram (arestas). Nesse contexto, tem-se:

$$\sum_{v \in V} \deg(v) = 2|E|$$

Assim, existem dez vértices e cada um possui grau nove. Portanto a soma dos graus é 90 e por isso o número de arestas deve ser 45.


---

### Exercício 01 (c)

Dado um grafo com 7 vértices; 3 deles de grau dois e 4 de grau um. Este grafo é conexo?

#### Resolução

Um grafo conexo é aquele que possui um caminho a partir de quaisquer pares de vértices. Para isso, o número mínimo de aresta é igual a 
$$n - 1$$. Sabendo que a soma dos graus dos vértices do grafo apresentado é dez e que, por consequência, a quantia de arestas é igual a cinco,
o grafo não pode ser considerado conexo, pois ele não possui, pelo menos, seis arestas.


---

### Exercício 01 (d)

Em um grupo de 5 pessoas, é possível que todos sejam amigos de exatamente 2 pessoas do grupo? E
quanto a 3 das pessoas no grupo?

#### Resolução

- Para duas pessoas:

  O somatório dos graus é igual a dez, logo o número de arestas é cinco, o que, para um grafo simples, é permitido.
  
- Para três pessoas:

  O somatório dos graus é igual a quinze, logo o número de arestas é 7,5; o que, para um grafo não é possível.

