# Teoria dos Grafos e Computabilidade - Aula 21

## Exercícios propostos

### 1. A aresta de menor peso sempre pertence à MST?

**Resposta:** **Verdadeiro.**

**Justificativa:**
A prova baseia-se na **Propriedade do Ciclo** e no método de contradição:
* Suponha que exista uma MST $T$ que não contenha a aresta de menor peso global $e_{min}$.
* Ao inserir $e_{min}$ em $T$, criamos obrigatoriamente um ciclo único.
* Como todos os pesos são distintos e $e_{min}$ é o menor de todos, qualquer outra aresta $e'$ pertencente a esse ciclo terá um peso maior ($w(e') > w(e_{min})$).
* Ao remover $e'$ e manter $e_{min}$, obtemos uma nova árvore geradora $T'$ com peso total $W(T') = W(T) - w(e') + w(e_{min})$.
* Visto que $w(e_{min}) < w(e')$, então $W(T') < W(T)$, o que contradiz o fato de $T$ ser mínima. Portanto, $e_{min}$ deve estar na MST.

---

### 2. A aresta de menor peso sempre está em um caminho de custo mínimo entre dois vértices?

**Resposta:** **Falso.**

**Justificativa:**
O caminho de custo mínimo (resolvido por algoritmos como Dijkstra) busca minimizar a **soma** dos pesos entre um par específico de vértices, o que não garante a inclusão da aresta de menor peso global do grafo.

**Exemplo de Contraexemplo:**
Considere um grafo com os vértices $\{A, B, C, D\}$ e as seguintes arestas:
* $(A, B)$ com peso 5
* $(B, C)$ com peso 6
* $(C, D)$ com peso 1 (Esta é a $e_{min}$ global)
* $(A, D)$ com peso 11

Ao buscar o caminho mínimo de $A$ para $D$:
* Caminho 1: $A 	o B 	o C 	o D$ (Custo total: $5 + 6 + 1 = 12$)
* Caminho 2: $A 	o D$ (Custo total: **11**)

O algoritmo escolherá o caminho direto $(A, D)$, ignorando a aresta $(C, D)$ de peso 1, provando que a aresta de menor peso nem sempre compõe um caminho mínimo entre dois pontos distantes.

---

### 3. Se G é uma árvore, a árvore de Dijkstra será igual à MST?

**Resposta:** **Verdadeiro.**

**Justificativa:**
* **Propriedade da Árvore:** Em qualquer árvore, existe **exclusivamente um único caminho** entre qualquer par de vértices $(u, v)$.
* **Consequência para Caminhos Mínimos:** Se só existe um caminho possível, o algoritmo de Dijkstra não tem alternativas para comparar; ele obrigatoriamente retornará esse único caminho existente.
* **Consequência para MST:** Uma Árvore Geradora Mínima de um grafo que já é uma árvore é a própria árvore original (visto que todas as arestas são pontes).
* **Conclusão:** Como ambas as estruturas (Árvore de Caminhos Mínimos e MST) devem conter os únicos caminhos que conectam todos os vértices sem formar ciclos, elas serão idênticas.