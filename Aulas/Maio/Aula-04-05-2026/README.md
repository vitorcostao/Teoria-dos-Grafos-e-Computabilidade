# Teoria dos Grafos e Computabilidade - Aula 23

## Exercícios propostos

### 1. Se $G$ é Euleriano, então $L(G)$ é Hamiltoniano?

**Resposta:** **Verdadeiro.**

**Justificativa:**
* Um grafo $G$ é Euleriano se possui um ciclo que percorre todas as suas arestas exatamente uma vez.
* A sequência de arestas $(e_1, e_2, ..., e_m)$ que forma o ciclo Euleriano em $G$ corresponde a uma sequência de vértices em $L(G)$.
* Como cada aresta de $G$ é visitada exatamente uma vez no ciclo, cada vértice de $L(G)$ será visitado exatamente uma vez na sequência correspondente.
* Como arestas consecutivas no ciclo de $G$ são adjacentes, os vértices correspondentes em $L(G)$ também serão adjacentes.
* Portanto, o ciclo Euleriano em $G$ induz diretamente um ciclo Hamiltoniano em $L(G)$.

---

### 2. Se $G$ é Hamiltoniano, então $L(G)$ é Euleriano?

**Resposta:** **Falso.**

**Justificativa:**
* Para que $L(G)$ seja Euleriano, todos os seus vértices devem ter grau par.
* O grau de um vértice em $L(G)$ (que representa uma aresta $e = \{u, v\}$ em $G$) é dado pela fórmula: $d_{L(G)}(e) = d_G(u) + d_G(v) - 2$.
* Ser Hamiltoniano em $G$ garante a existência de um ciclo que visita todos os vértices, mas não impõe que a soma dos graus das extremidades de cada aresta ($d_G(u) + d_G(v)$) seja sempre par.
* **Contraexemplo:** Considere um grafo Hamiltoniano onde um vértice tem grau 3 e seu vizinho tem grau 2. A aresta $e$ entre eles terá grau $3 + 2 - 2 = 3$ em $L(G)$. Como o grau é ímpar, $L(G)$ não pode ser Euleriano.

---

### 3. Se $G$ é Euleriano, então $L(G)$ é Euleriano?

**Resposta:** **Verdadeiro.**

**Justificativa:**
* Sabemos que em $G$ (Euleriano), todos os vértices têm grau par. Seja $d_G(v) = 2k$ para todo $v \in V$.
* O grau de um vértice em $L(G)$ é $d_{L(G)}(e) = d_G(u) + d_G(v) - 2$.
* Se todos os vértices em $G$ têm grau par, então a soma de dois graus pares é sempre par ($2k + 2m$). Subtraindo 2, o resultado permanece par.
* **Conclusão:** Se $G$ é Euleriano, todos os vértices de $L(G)$ terão grau par. Se $L(G)$ for conectado, então $L(G)$ também será Euleriano. 