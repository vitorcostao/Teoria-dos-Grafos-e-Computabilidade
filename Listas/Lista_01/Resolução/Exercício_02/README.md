# Teoria dos Grafos e Computabilidade - Lista 02


## Exercício 02 (a)

Liste todos os grafos que possuem {a, b, c} como seu conjunto de vértices. Organize a lista de forma que
sejam ilustrados o grafo e o seu complemento (um ao lado do outro)

### Resolução

<img width="1064" height="1280" alt="image" src="https://github.com/user-attachments/assets/c82254cd-20da-444c-8283-c7c7296d00d4" />

---

## Exercício 02 (b)

Encontre o número de vértices e arestas em cada um dos grafos (simples) e não-direcionados:

### Resolução

- Grafo nulo $$N_n$$: n vértices e zero arestas.
- Grafo ciclo $$C_n$$: n vértices e n arestas.
- Grafo completo $$K_n$$: n vértices e $$\frac{n(n-1)}{2}$$
- Grafo bipartido completo $$K_n,_m$$: n + m vértices e n * m arestas

---

## Exercício 02 (c)

Seja $G$ um grafo simples com pelo menos dois vértices. Prove que $G$ deve conter pelo menos dois vértices de mesmo grau.

### Resolução

- Em um grafo simples com $n$ vértices:
  - O número de arestas está entre 0 e $\frac{n(n-1)}{2}$.
  - O grau de cada vértice está entre 0 e $n-1$.

- Observações importantes:
  - Se houver um vértice de grau 0 (isolado), não pode existir um vértice de grau $n-1$, pois este estaria conectado a todos os outros, incluindo o isolado — o que é impossível.
  - Se houver um vértice de grau $n-1$, então não pode haver vértice de grau 0, pelo mesmo motivo.

- Conclusão:
  - Os possíveis graus distintos são, no máximo, $n-1$ valores.
  - Como temos $n$ vértices, pelo **Princípio da Casa dos Pombos**, pelo menos dois vértices devem ter o mesmo grau.


---

## Exercício 02 (d)

Uma string binária é uma sequência finita de 0s e 1s. O comprimento de uma string binária é o núumero
total de símbolos que ocorrem nela.

### Resolução

<img width="1600" height="981" alt="image" src="https://github.com/user-attachments/assets/d99de5c8-7d60-4466-af0e-0ae91ebad00a" />



