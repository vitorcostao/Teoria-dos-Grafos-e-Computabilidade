# Teoria dos Grafos e Computabilidade - Aula 05

## Problema proposto

Seja $G = (V, E)$ um grafo direcionado. Seja $v \in V$ um vértice qualquer. Projete uma solução para encontrar todos os vértices que são alcançados por $v$. 
E como ficaria para encontrar os vértices que alcançavam $v$.

## Resolução

Para saber quais vértices podem ser alcançados a partir de um vértice qualquer é preciso usar o algoritmo de busca em largura (BFS). 

### 1) Estrutura básica

O BFS geralmente utiliza uma fila (queue) para controlar a ordem de visitação dos vértices:

- Vértices visitados são marcados para não serem visitados novamente.
- Vértices são enfileirados quando encontrados, mas ainda não visitados.

### 2) Passos do algoritmo

1. **Escolher ponto de partida:**  
   Pegue um vértice inicial `v0` e marque como visitado.

2. **Preparar a fila:**  
   Coloque `v0` em uma fila.

3. **Percorrer os vértices:**  
   Enquanto a fila não estiver vazia:
   - Tire o vértice da frente da fila (`v`).
   - Olhe todos os vértices ligados a `v` (adjacentes).
   - Para cada vértice adjacente `u`:
     - Se `u` ainda não foi visitado:
       - Marque `u` como visitado.
       - Coloque `u` no final da fila.

4. **Continuar até o fim:**  
   Repita o processo até visitar todos os vértices que puder alcançar.

> Para obter os vértices alcançados, basta inicializar um conjunto em que todo vértice visitado deve ser adicionado nele.

### 3) Vértices que alcançavam $v$

Para determinar os vértices que alcançavam $v$ basta inverter as arestas do grafo (Grafo Transposto) e aplicar o BFS.

---

## Novos conceitos

#### Fecho transitivo direto

- O fecho transitivo direto de um vértice 𝑣 é o conjunto de todos os vértices que podem ser alcançados a partir de 𝑣.

#### Fecho transitivo inverso

- O fecho transitivo inverso de 𝑣 é o conjunto de todos os vértices que podem chegar a 𝑣.