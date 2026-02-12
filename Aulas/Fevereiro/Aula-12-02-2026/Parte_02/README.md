# Teoria dos Grafos e Computabilidade - Aula 02 - Parte 02

## Problema Proposto



<img width="300" height="300" alt="image" src="https://github.com/user-attachments/assets/84c7b6bc-3107-4e84-aa77-652562ab42df" />

Considere o grafo completo representado acima, enumere todos os subgrafos de G:

---

### Definição Matemática

Dado um grafo $G = (V, E)$, um grafo $H = (V', E')$ é considerado um subgrafo de $G$ se, e somente se:
1. $V' \subseteq V$
2. $E' \subseteq E$

Para o grafo em questão, temos o conjunto de vértices $$V = \lbrace 1, 2, 3 \rbrace $$.

---

### Resolução

Para encontrar a quantidade total de subgrafos, é preciso analisar as combinações de vértices e as possíveis arestas entre eles.

#### 1. Subgrafos com 1 Vértice 
Existem três combinações possíveis para um grafo com apenas um vértice, sendo elas:
* $H_1 = (\lbrace 1 \rbrace, \emptyset)$
* $H_2 = (\lbrace 2 \rbrace, \emptyset)$
* $H_3 = (\lbrace 3 \rbrace, \emptyset)$
> OBS: Não existem arestas para grafos com apenas um vértice

#### 2. Subgrafos com 2 Vértices 
Existem três combinações possíveis de vértices para um grafo com V tendo três elementos:
* Vértices $\lbrace 1, 2 \rbrace$
* Vértices $\lbrace 1, 3 \rbrace$
* Vértices $\lbrace 2, 3 \rbrace$

Em se tratando de arestas, é possível ter duas possibilidades por par (0 ou 1), o que configura num total de seis possibilidades.

#### 3. Subgrafos com 3 Vértices ($k=3$)
Existe apenas uma combinação possível com três vértices baseado no conjunto V. Além disso, cada aresta pode estar presente ou não o que totaliza em oito possibilidades.

Os conjuntos de arestas possíveis para $V' = \lbrace 1, 2, 3 \rbrace$ são:
* $\emptyset$ (Grafo vazio)
* $\lbrace (1,2) \rbrace, \lbrace (1,3) \rbrace, \lbrace (2,3) \rbrace$ (1 aresta)
* $\lbrace (1,2), (1,3) \rbrace, \lbrace (1,2), (2,3) \rbrace, \lbrace (1,3), (2,3) \rbrace$ (2 arestas)
* $\lbrace (1,2), (1,3), (2,3) \rbrace$ (O próprio grafo $G$)

### Conclusão
O número total de subgrafos para $G$ é a soma dos casos acima: $3 + 6 + 8 = 17$ subgrafos.

---
<p align="center">
$$\text{Número de arestas possíveis} = \binom{k}{2} = \frac{k \cdot (k-1)}{2}$$

$$\text{Número de subgrafos} = 2^{\binom{k}{2}}$$
</p>
​
