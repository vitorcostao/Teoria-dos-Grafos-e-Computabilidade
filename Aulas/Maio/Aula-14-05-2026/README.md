# Teoria dos Grafos e Computabilidade - Aula 24

## Aula 24: Fluxo Máximo e Algoritmos de Otimização em Redes

### 1. Fluxo Máximo usando *Widest Path* (Estratégia de Edmonds-Karp Modificada)

A estratégia do "Caminho Mais Largo" é uma variante do método de caminhos aumentantes que, em cada iteração, escolhe o caminho com a **maior capacidade residual** (o gargalo mais largo). Isso visa reduzir o número total de iterações necessárias para atingir o fluxo máximo.

**Grafo de Entrada:**
* $S 	o A$ (Capacidade: 4)
* $S 	o B$ (Capacidade: 2)
* $A 	o B$ (Capacidade: 3)
* $A 	o C$ (Capacidade: 5)
* $B 	o C$ (Capacidade: 3)
* *Fonte:* $S$ | *Sumidouro:* $C$

**Simulação das Iterações:**

1.  **Encontrar o Widest Path:** O caminho $S 	o A 	o C$ tem capacidade 4 (limitado pelo arco $S 	o A$).
    * Fluxo enviado: 4.
    * Capacidades residuais: $S 	o A = 0$, $A 	o C = 1$.
2.  **Próximo Caminho:** O único caminho restante com capacidade é $S 	o B 	o C$.
    * Capacidade residual do caminho: 2.
    * Fluxo enviado: 2.
    * Capacidades residuais: $S 	o B = 0$, $B 	o C = 1$.
3.  **Resultado:** Não há mais caminhos de $S$ para $C$.
    * **Fluxo Máximo Total: $4 + 2 = 6$.**

---

### 2. Algoritmo de Ford-Fulkerson

O algoritmo de **Ford-Fulkerson** é um método genérico para calcular o fluxo máximo em uma rede de fluxo. Ele utiliza o conceito de **Grafo Residual** e **Caminhos Aumentantes**.

**Grafo de Entrada (Ajustado):**
*Considerando a alteração solicitada ($A 	o C = 2$):*
* $S 	o A$ (4), $S 	o B$ (2)
* $A 	o B$ (3), $A 	o C$ (**2**)
* $B 	o C$ (3)

**Explicação do Processo:**

1.  **Busca de Caminho Aumentante:** O algoritmo busca qualquer caminho da fonte ao sumidouro onde todas as arestas tenham capacidade residual $> 0$.
2.  **Cálculo do Gargalo:** Identifica-se a aresta de menor capacidade no caminho escolhido. Este valor é o máximo que podemos enviar por esta rota.
3.  **Atualização da Rede Residual:** * Subtrai-se o valor do gargalo das capacidades das arestas no sentido do fluxo.
    * Soma-se o valor do gargalo às "arestas reversas" (capacidade de cancelamento de fluxo).
4.  **Repetição:** O processo se repete até que não existam mais caminhos aumentantes.

**Simulação com $A 	o C = 2$:**

1.  **Caminho $S 	o A 	o C$:** Gargalo é 2 (limitado por $A 	o C$).
    * Fluxo acumulado: 2.
2.  **Caminho $S 	o A 	o B 	o C$:** Gargalo é 2 (arestas restantes: $S 	o A=2, A 	o B=3, B 	o C=3$).
    * Fluxo acumulado: $2 + 2 = 4$.
3.  **Caminho $S 	o B 	o C$:** Gargalo é 1 (arestas restantes: $S 	o B=2, B 	o C=1$).
    * Fluxo acumulado: $4 + 1 = 5$.
4.  **Resultado:** Não há mais caminhos.
    * **Fluxo Máximo Total: 5.**

---

### 3. Teorema Min-Cut Max-Flow

O valor do fluxo máximo encontrado (5 no segundo caso) é igual à capacidade do **Corte Mínimo** do grafo. O corte que separa $S$ de $C$ com menor soma de capacidades de arestas que saem do conjunto de $S$ define o gargalo total da rede.