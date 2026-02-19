# Teoria dos Grafos e Computabilidade - Aula 03

## Problema proposto

Seja G = (v, e) um grafo não-direcionado. Calcule o número mínimo e máximo arestas.

**Resolução:** Para esse problema, o número mínimo de arestas possíveis para um grafo é zero.
Em se tratando do número máximo, para um grafo não simples, a resposta é infinito, porém caso seja simples,
a resposta é obtida pela fórmula:

$$\text{Número de arestas} = \frac{n(n - 1)}{2}$$

$$\text{Número de subgrafos} = \sum_{i=1}^{n} \binom{n}{i} \cdot 2^{\frac{n(n-1)}{2}}$$

---

## Novos conceitos

- **Grau:** Trata-se do número de arestas incidentes sobre um vértice.
- **Vértice pendente:** É aquele cujo o Grau possui valor um.
- **Vértice isolado:** É aquele cujo o Grau possui valor zero.
- **Caminho**: Tem-se por caminho uma sequência alternada de vértices e arestas.

> OBS: Um caminho simples não possui repetições.

---

## Componentes conexos de um grafo

Um grafo é dito como conexo caso seja possível traçar um caminho entre quaisquer dois de seus vértices, 
utilizando arestas do grafo. Quando um grafo não é conexo, ele pode ser dividido em componentes
conexos. Cada componente conexo é um subgrafo conexo maximal. Isso significa que ele contém o 
maior conjunto possível de vértices que podem ser alcançados entre si.

---

## Teorema do Aperto de Mão

$$\sum_{v \in V} \text{deg}(v) = 2|E|$$

> O somatório dos graus é igual ao dobro do número de arestas.

---

## Problema proposto

Prove que o número de vértices de grau impar é par para um grafo simples e não-direcionado.

**Resolução:** Utilizando o Teorema do Aperto de Mão, é possível utilizar as propriedades do somatório 
para desmembra-lo e provar matematicamente a afirmação.

<img width="460" height="100" alt="image" src="https://github.com/user-attachments/assets/29577bad-1b3a-4237-88e4-0e380a6a6fbc" />

> Como o resultado precisa ser par, obrigatoriamente o somatório dos vértices impares deve ser par.

