# Teoria dos Grafos e Computabilidade - Aula 02 - Parte 01

## Problema proposto - 01

A imagem abaixo representa um mapa de uma cidade. A cidade contém três pontos de interesse sendo eles P1, P2 e P3. Desse modo, crie um grafo para representar o mapa e represente-o
através de um desenho e através da forma matemática.

<p align="center">
<img width="300" height="300" alt="image" src="https://github.com/user-attachments/assets/cabb3c20-267a-4d05-ad7e-9a57643807f3" />
</p>

### Resolução

Considerando que a cidade não possui restrição de sentido entre os pontos de interesse e que, para chegar do ponto P1 até o ponto P3, é necessário passar por P2, é possível tirar algumas conclusões sobre as 
restrições dos grafos.

A primeira é que a ordem não importa, pois é possível sair de P1 para P2, ou vice versa, passando pela direita ou pela esquerda. 
Além disso, o trajeto P2-P3 é uma "via de mão dupla", ou seja, existe apenas um caminho entre os pontos em questão. 

A segunda é que não há loops, pois o grafo em questão não permite que alguém saia do ponto P1 e retorne a ele mesmo. Desse modo, tem-se abaixo a representação imagética e matemática do grafo em questão: 


<p align="center">
  <img src="https://github.com/user-attachments/assets/5b89e4b8-71b4-46a9-8be8-b1fa4347e73d" width="300">
</p>


$$G = (V, E)$$
  
$$V = \{p1, p2, p3}\$$
  
$$E \subseteq V \times V$$

$$E = \lbrace \lbrace P_1, P_2 \rbrace, \lbrace P_1, P_2 \rbrace, \lbrace P_2, P_3 \rbrace \rbrace$$

### Análise do problema

No exercício acima, existem duas possibilidades de rotas para P1-P2, sendo elas direita ou esquerda. Nesse contexto, é importante frizar que em termos de conjuntos, não é possível representar um par não ordenado em dois sentidos,
pois a ordem não importa nesse caso. Portanto, faz-se necessário, no conjunto de relação E, exibir $$\lbrace P_1, P_2 \rbrace$$ duas vezes para que tal questão seja expressa.


## Definições: Alguns Tipos de Grafos

### Grafo Simples

Um grafo é dito **simples** quando não há loops ou arestas paralelas, ou seja:

- Não pode existir aresta de um vértice para ele mesmo.
- Não pode existir mais de uma aresta entre o mesmo par de vértices.

**Representação:**

G = (V, E)

---

### Multigrafo

Um **multigrafo** é aquele que pode possuir:

- Loops (arestas que ligam um vértice a ele mesmo)
- Arestas paralelas (mais de uma aresta entre o mesmo par de vértices)

Desse modo, todo grafo simples é um multigrafo, mas nem todo multigrafo é um grafo simples.

**Representação:**

G = (V, E)

---

### Grafo Rotulado

Um **grafo rotulado** é aquele em que arestas e/ou vértices recebem rótulos (labels) para melhor representar o problema.

Considerando rótulos nas arestas:

**Representação:**

(G, Le)

Onde:

- G = (V, E)  
- Le: E → S  
  - Função que associa cada aresta a um rótulo pertencente ao conjunto S

---

### Grafo Ponderado

Um **grafo ponderado** é aquele em que vértices ou arestas recebem pesos, geralmente números inteiros.

Considerando pesos nas arestas:

**Representação:**

(G, w)

Onde:

- G = (V, E)  
- w: E → I  
  - Função que associa cada aresta a um peso inteiro pertencente ao conjunto I

---

### Grafo Rotulado e Ponderado

É o grafo que possui simultaneamente rótulos e pesos associados às arestas.

**Representação:**

(G, Le, w)

Onde:

- G = (V, E)  
- Le: E → S  
- w: E → I  




