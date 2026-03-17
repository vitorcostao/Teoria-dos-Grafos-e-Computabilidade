# Teoria dos Grafos e Computabilidade - Lista 01

## Análise de alguns grafos especiais

### 1) Para quais valores de n os grafos abaixo são regulares

$K_n$ (Grafo completo)

$C_n$ (Grafo ciclo)

$Q_n$ (Grafo cubo)

$W_n$ (Grafo roda)

### 2) O grafo complementar G´ de um grafo simples G tem os mesmos vértices de G. Dois vértices são adjacentes em G se, e somente se, eles não são adjacentes em G´. Determine os seguintes grafos.


$K_n´$

$K_{m,n}´$

$C_n´$

$Q_n´$

### 3) O grafo tripartite completo $K_{r,s,t}$ consiste de três conjuntos de vértices de tamanhos $r$, $s$ e $t$, com arestas unindo dois vértices se, e somente se, eles pertencem a conjuntos distintos.

3.1) Desenhe os grafos $K_{2,2,2}$ e $K_{2,3,3}$

3.2) Quantos vértices e arestas o grafo $K_{r,s,t}$ possui (exprima sua resposta em função de r, s e t)?

3.3) Qual é o complemento de $K_{r,s,t}$?

--- 

### Resolução 01

O grafo regular é aquele que os vértices possuem o mesmo grau. Desse modo, é importante entender as definições de cada um dos grafos.

1) Grafo Completo: É aquele que todos os vértices estão conectados entre si.

2) Grafo Ciclo: É aquele que possui um único ciclo fechado.

3) Grafo Cubo: É aquele que dois vértices são adjacentes se e somente se seus vetores diferem por um bit.

4) Grafo Roda: É aquele que é formado a partir de um ciclo $C_{n-1}$.

Nesse contexto, para que seja regular, o grafo completo n deve ser maior ou igual a um, e o ciclo deve possuir um valor de n maior ou igual a três. Em se tratando do grafo cubo, o valor de n deve ser maior que um para que ele seja regular. Por fim, o grafo roda não é regular para nenhum valor de n, pois o vértice central sempre será o de maior grau.

---

### Resolução 02

O grafo complementar de $K_n$ é aquele formado apenas pelos vértices de K e nenhuma aresta. O grafo $K_{m,n}$ é um bipartido completo, ou seja, todos os vértices de $V_1$ estão ligados a $V_2$, desse modo o seu complemento será um grafo composto por dois componentes conexos que ligam em que os vértices de $V_1$ se ligam e os de $V_2$ também se ligam. O complemento do grafo ciclo é representado pela ligação dos vértices que anteriormente não eram adjacentes. Por fim, o complemento grafo cubo possui como ligações aqueles vértices que não possuem diferença de um bit entre si.

--- 

### Resolução 03

#### Tripartite 2, 2, 2

<img width="676" height="586" alt="image" src="https://github.com/user-attachments/assets/0de3a5e7-db5a-431a-9063-d06cb8c9f69f" />

#### Tripartite 2, 2, 3

<img width="527" height="577" alt="image" src="https://github.com/user-attachments/assets/53467e82-2182-483f-933e-01c18aff06a9" />

#### Vértices e arestas em função de r, s e t

Os número de vértices de um grafo tripartite completo são expressos por |r| + |s| + |t| vértices e o número de arestas pode ser expresso por: 

$$|E(K_{r,s,t})| = \frac{\sum_{v \in V} \deg(v)}{2} = \frac{r(s+t) + s(r+t) + t(r+s)\}{2}$$

De modo simples, como o número de vértices de cada conjunto está definido em termos de valores e cada conjunto não pode se ligar a ele mesmo, logo um determinado conjunto possuirá como arestas a sua quantia de vértices multiplicado pelo tamanho dos outros conjuntos dividido por dois. 

#### O complemento de $$K_{r, s, t}$$

De modo simples, o complemento é um grafo que liga apenas vértices que estão no mesmo conjunto.