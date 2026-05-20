# Teoria dos Grafos e Computabilidade - Aula 25

## Conceitos sobre fluxos e Algoritmo de Ford-Fulkerson

### Redes de Fluxo

Uma **rede de fluxo** é um grafo dirigido $G = (V, E)$ onde cada aresta $(u, v) \in E$ possui uma **capacidade** $c(u, v) \geq 0$. São definidos dois vértices especiais:

- **Fonte** ($s$): vértice de origem, onde o fluxo entra na rede.
- **Sumidouro** ($t$): vértice de destino, onde o fluxo sai da rede.

O **fluxo** $f(u, v)$ em uma aresta representa a quantidade de "material" que passa por ela, respeitando as seguintes propriedades:

1. **Restrição de capacidade:** O fluxo em cada aresta não pode exceder sua capacidade:
$$0 \leq f(u, v) \leq c(u, v), \quad \forall (u, v) \in E$$

2. **Conservação de fluxo:** Para todo vértice $v \in V \setminus \{s, t\}$, a soma dos fluxos de entrada deve ser **igual** à soma dos fluxos de saída:
$$\sum_{u \in V} f(u, v) = \sum_{w \in V} f(v, w)$$

> **Importante:** A conservação de fluxo impõe igualdade entre fluxos de entrada e saída em cada vértice intermediário. No entanto, **não existe uma relação obrigatória entre os fluxos e as capacidades das arestas** — uma aresta pode ter capacidade 10 e transportar fluxo 3, por exemplo. A única exigência é que o fluxo não ultrapasse a capacidade.

O **valor do fluxo** $|f|$ da rede é definido como o total que sai da fonte:
$$|f| = \sum_{v \in V} f(s, v)$$

---

### Grafo Residual

O **grafo residual** $G_f = (V, E_f)$ representa a capacidade restante de fluxo em cada aresta após um fluxo $f$ ter sido atribuído à rede original.

Para cada aresta $(u, v) \in E$ com capacidade $c(u, v)$ e fluxo $f(u, v)$, o grafo residual possui:

- Uma **aresta direta** $(u, v)$ com **capacidade residual**:
$$c_f(u, v) = c(u, v) - f(u, v)$$
(indica o quanto ainda pode ser enviado de $u$ para $v$)

- Uma **aresta reversa** $(v, u)$ com capacidade residual:
$$c_f(v, u) = f(u, v)$$
(indica o quanto do fluxo já enviado pode ser "cancelado")

As aresetas com capacidade residual zero são omitidas de $G_f$. O grafo residual é fundamental porque permite identificar **caminhos aumentantes** — caminhos de $s$ a $t$ nos quais ainda é possível aumentar o fluxo.

---

### Algoritmo de Ford-Fulkerson

O algoritmo de **Ford-Fulkerson** é um método iterativo para encontrar o **fluxo máximo** em uma rede de fluxo. A ideia central é encontrar sucessivamente caminhos aumentantes no grafo residual e aumentar o fluxo ao longo desses caminhos até que nenhum caminho de $s$ a $t$ exista mais em $G_f$.

#### Funcionamento

1. **Inicialização:** Define $f(u, v) = 0$ para todas as arestas.
2. **Construção do grafo residual** $G_f$ a partir do fluxo atual.
3. **Busca de caminho aumentante:** Encontra um caminho $p$ de $s$ a $t$ em $G_f$ (usando BFS ou DFS).
4. **Cálculo da capacidade do gargalo:** Determina a menor capacidade residual ao longo do caminho:
$$c_f(p) = \min_{(u,v) \in p} c_f(u, v)$$
5. **Atualização do fluxo:** Para cada aresta $(u, v) \in p$:
   - Aumenta $f(u, v)$ em $c_f(p)$ (aresta direta)
   - Reduz $f(v, u)$ em $c_f(p)$ (aresta reversa)
6. **Repete** os passos 2–5 até não existir mais caminho aumentante.

#### Pseudocódigo

```
Ford-Fulkerson(G, s, t):
  para cada aresta (u, v) em G:
    f(u, v) = 0

  enquanto existe caminho p de s a t em G_f:
    c_f(p) = min { c_f(u,v) : (u,v) em p }
    para cada aresta (u, v) em p:
      f(u, v) = f(u, v) + c_f(p)
      f(v, u) = f(v, u) - c_f(p)

  retorna f
```

#### Complexidade

| Variante | Estratégia de busca | Complexidade |
|---|---|---|
| Ford-Fulkerson genérico | DFS | $O(E \cdot |f^*|)$ |
| Edmonds-Karp | BFS (caminho mais curto) | $O(V \cdot E^2)$ |

onde $|f^*|$ é o valor do fluxo máximo. O uso de BFS (variante de Edmonds-Karp) garante terminação mesmo para capacidades irracionais.

---

### Teorema do Fluxo Máximo e Corte Mínimo

Um resultado fundamental da teoria de fluxos é o **Teorema Max-Flow Min-Cut**:

> O valor do fluxo máximo de $s$ a $t$ em uma rede é **igual** à capacidade do corte mínimo que separa $s$ de $t$.

Um **corte** $(S, T)$ particiona os vértices de $V$ em dois conjuntos $S \ni s$ e $T \ni t$. A **capacidade do corte** é:
$$c(S, T) = \sum_{u \in S,\ v \in T} c(u, v)$$

Ford-Fulkerson termina exatamente quando o fluxo atual é igual à capacidade do corte mínimo, certificando a otimalidade da solução.