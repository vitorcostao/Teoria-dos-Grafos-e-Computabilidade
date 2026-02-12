# Teoria dos Grafos e Computabilidade - Aula 01

## Definições: O que é um conjunto?

Um conjunto é definido como uma relação de elementos que podem ser expressados de alguma forma. Desse modo, matematicamente falando, os conjuntos são representados por letras maiúsculas e seus elementos entre chaves **{}**, por exemplo: **A = {1, 2, 3}**. Além disso, é possível estabelecer relações de pertinência e continência entre conjuntos e incluir restrições:

### Pertinência

  - A = {x e N}
  - A´ = {x e N / 1 <= x <= 10}

### Continência

  - B c A
  - A c B

### Quando conjuntos são iguais?

Em termos práticos, um conjunto A é igual a um conjunto B caso A esteja contido em B e o conjunto B esteja contido em A, além disso a ordem dos elementos dispostos em um conjunto não importa para determinação de igualdade de conjuntos. Já em se tratando de tuplas, que são pares ordenados, a ordem dos elementos importa e tais elementos são dispostos entre parênteses.


## Definições: O que é um grafo?

Tendo a definição de conjuntos exposta, um grafo G é a relação de um conjunto V por um conjunto E, em que V é o conjunto de elementos(vértices) e E é um conjunto de relações de elementos(arestas). Desse modo:

  $$G = (V, E)$$
  
  $$V = \{1, 2, 3}\$$
  
  $$E \subseteq V \times X$$
  
  $$E = \{ (v_1, v_2) \mid v_1 \in V, v_2 \in V \}$$

### Grafo direcionado (Ordem importa)

$$E = \{ \(v_1, v_2)\ \mid v_1, v_2 \in V, v_1 \neq v_2 \}$$

> OBS: A condição de diferença implica na não repetição de tuplas, ou seja, uma entre as tuplas será escolhida [ (1, 2) ou (2, 1) ]

### Grafo não direcionado (Ordem não importa)

$$E = \{ \{v_1, v_2\} \mid v_1, v_2 \in V \}$$

> OBS: A representação por chaves implica na não importância da ordem

---

## Problema real (Campeonato Brasileiro)

Pense no Brasileirão, para representar jogos do campeonato, é possível usar um grafo em que o conjunto V será os times(vértices) e o conjunto E será o mando de campo(arestas). Desse modo, a ordem necessáriamente importa caso a
representação seja de todos os jogos da competição, pois, em uma partida entre Atlético e Cruzeiro, o mandante deve aparecer primeiro na tupla (CAM x CRU é diferente de CRU x CAM). Assim, tem-se:

$$V = \{ \text{Atlético, Cruzeiro, Flamengo, Palmeiras, ...} \}$$

$$E \subseteq V \times V$$

$$e = (v_{mandante}, v_{visitante})$$

$$E = \{ (v_1, v_2) \mid v_1, v_2 \in V, v_1 \neq v_2 \}$$