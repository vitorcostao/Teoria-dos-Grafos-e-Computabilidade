# Exercício 04

## Problema

O objetivo é encontrar, entre dois vértices `s` e `t`, um caminho que **maximize o peso da aresta mais fraca (o gargalo)**. Em vez de somar pesos (como no caminho mínimo), queremos que o menor elo da nossa corrente seja o maior possível.

---

### 1. A Lógica Central
A função de custo de um caminho aqui não é a soma, mas sim o seu **gargalo**.
Para qualquer caminho, definimos a sua "largura" como:
$$\text{Largura} = \min(\text{pesos das arestas do caminho})$$

Queremos escolher o caminho que maximize esse valor.

---

### 2. O Algoritmo de Relaxamento

Para cada vértice `v`, mantemos `d[v]`, que é a **melhor largura encontrada até ao momento para chegar a `v`**.

* **Inicialização:**
    * `d[s] = ∞` (fonte acessível sem restrições).
    * `d[v] = 0` para todos os outros vértices.
* **O Passo de Relaxamento:**
    Ao analisar uma aresta `(u, v)` com peso `w`:
    1.  Calculamos a largura do caminho candidato passando por `u`: `candidato = min(d[u], w)`.
    2.  Se `candidato > d[v]`, então encontrámos um caminho melhor.
    3.  Atualizamos: `d[v] = candidato`.

> **Fórmula de Atualização:**
> `d[v] = max(d[v], min(d[u], w))`

---

### 3. Comparativo: Caminho Mínimo vs. Caminho Mais Largo

A tabela abaixo mostra como a estrutura lógica do Dijkstra clássico é adaptada para este problema:

| Característica | Dijkstra (Caminho Mínimo) | Widest Path (Caminho Largo) |
| :--- | :--- | :--- |
| **Objetivo** | Minimizar a soma | Maximizar o gargalo |
| **Combinação** | Soma: `d[u] + w` | Mínimo: `min(d[u], w)` |
| **Critério de Update** | `novo < d[v]` | `novo > d[v]` |
| **Relaxamento** | `d[v] = min(d[v], d[u] + w)` | `d[v] = max(d[v], min(d[u], w))` |

---

### 4. Por que o BFS funciona aqui?

Diferente do BFS padrão (que visita cada nó uma única vez), aqui **um vértice pode ser inserido na fila várias vezes**.

* **Propagação:** Sempre que descobrimos um caminho mais "largo" para um vértice, precisamos avisar os seus vizinhos, pois eles podem beneficiar dessa nova descoberta.
* **Monotonicidade:** Como `d[v]` só aumenta ao longo do tempo (nunca pioramos uma solução), o algoritmo eventualmente estabiliza.
* **Terminação:** Como existem um número finito de arestas e, consequentemente, um número finito de possíveis valores de "gargalo", o algoritmo garante a convergência.

---

### 5. Resumo para Implementação

1.  Coloque `s` na fila.
2.  Enquanto a fila não estiver vazia:
    * Remova o vértice `u`.
    * Para cada vizinho `v` com aresta de peso `w`:
        * `largura_candidata = min(d[u], w)`
        * Se `largura_candidata > d[v]`:
            * `d[v] = largura_candidata`
            * Coloque `v` na fila (para que ele atualize os seus próprios vizinhos).

> **Dica:** Se o grafo for muito grande, o uso de uma **Fila de Prioridade (Max-Heap)**, baseada na ideia do Algoritmo de Prim ou Dijkstra, torna a execução muito mais eficiente, processando primeiro os caminhos que já possuem as maiores larguras.
