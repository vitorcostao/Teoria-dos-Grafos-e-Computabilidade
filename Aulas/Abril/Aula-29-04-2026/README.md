# Teoria dos Grafos e Computabilidade - Aula 22

## Exercícios propostos

---

### 1. Identificação de Grafo Euleriano (Ciclo Euleriano)

**Critério:**
Um grafo conectado é **Euleriano** se, e somente se, todos os seus vértices possuírem **grau par**.

**Justificativa:**
* Para que um ciclo passe por todas as arestas exatamente uma vez e retorne ao ponto de origem, cada vez que o caminhamento "entra" em um vértice por uma aresta, ele deve obrigatoriamente "sair" por outra. 
* Portanto, as arestas devem sempre vir em pares para cada vértice, resultando em um grau par ($ext{grau}(v) \pmod 2 = 0$).

---

### 2. Identificação de Grafo Hamiltoniano

A identificação de ciclos hamiltonianos (que visitam cada vértice exatamente uma vez) é um problema complexo, mas existem condições suficientes:

* **Teorema de Dirac:** Se um grafo $G$ possui $n \ge 3$ vértices e cada vértice possui $	ext{grau}(v) \ge n/2$, então o grafo é Hamiltoniano.
* **Teorema de Ore:** Se para cada par de vértices não adjacentes $u$ e $v$, a soma de seus graus for $	ext{grau}(u) + 	ext{grau}(v) \ge n$, então o grafo é Hamiltoniano.

*Nota: Estas são condições suficientes, mas não necessárias. Um grafo pode ser Hamiltoniano mesmo sem atender a Dirac ou Ore.*

---

### 3. Algoritmo para Caminhamento Euleriano (Algoritmo de Fleury)

**Abordagem:**
O caminhamento euleriano utiliza uma busca que prioriza a preservação da conectividade do grafo restante.

**Regra Fundamental:**
Ao escolher a próxima aresta a partir de um vértice atual, deve-se:
1.  Utilizar qualquer aresta disponível que **não seja uma ponte** (uma aresta cuja remoção desconecta o grafo).
2.  Somente atravessar uma ponte se não houver outra alternativa.

**Resultado:**
Essa estratégia (frequentemente implementada via DFS com verificação de pontes) garante que o algoritmo não "fique preso" em um componente isolado antes de visitar todas as arestas das outras partes do grafo.

---

### 4. Caminhamento Hamiltoniano e Complexidade

**Definição:**
O problema consiste em encontrar um caminho que visite cada vértice do grafo exatamente uma vez.

**Análise de Complexidade:**
* **Problema NP-Completo:** Diferente do caminho Euleriano (que possui solução em tempo linear $O(E)$), não existe um algoritmo conhecido que resolva o caminho Hamiltoniano em tempo polinomial para todos os casos.
* **Implicação:** À medida que o número de vértices aumenta, o tempo necessário para encontrar a solução (ou provar que ela não existe) cresce exponencialmente, tornando-o um dos problemas clássicos da computação teórica