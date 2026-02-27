# Teoria dos Grafos e Computabilidade - Aula 06

## Ordenação Topológica

Seja G = (V, E) um grafo direcionado sem ciclos, projete uma solução para ordenar os vértices de forma que 
cada aresta tenha caminho de $$u$$ para $$v$$ e o vértice $$u$$ apareça antes de $$v$$.

<img width="1271" height="566" alt="image" src="https://github.com/user-attachments/assets/1d1e720d-b8dd-4b11-995c-a7310ed5aef3" />


## Resolução

### Ordenação Topológica utilizando busca em profundidade.

Para projetar a solução, primeiro é preciso entender como funciona o algoritmo DFS:

1) Todos os vértice inicializam como não visitados(Branco).
2) Escolher um vértice aleatório e marcá-lo como em processo(Cinza).
3) A partir desse nó inicial, explorar um vizinho não visitado. Continuar até achar um nó sem vizinhos.
4) Qaundo encontrar um nó sem vizinhos, marcá-lo como finalizado(Preto) e retornar para nós anteriores.
5) Caso nós anteriores não possuam vizinhos visitados, marcá-los como finalizado até encontrar nó que possua vizinho não visitado.
6) O algoritmo termina até que todos os nós sejam processados e finalizados(Preto) ou quando algum objetivo é alcançado.

Tendo o passo a passo em mente, é possível utilizar números para determinar quando cada vértice foi visitado e finalizado de modo que, ao terminar
a numeração, os maiores números são aqueles que começarão no ínicio da ordenação como mostra a imagem abaixo.

<img width="1141" height="499" alt="image" src="https://github.com/user-attachments/assets/43f14bee-35ba-44bc-8bb0-5333c901f533" />

---

### Ordenação Topológica utilizando algoritmo de Kahn

Para projetar a solução, primeiro é preciso entender como funciona o algoritmo de Kahn:

1) Insere na fila os vértices de grau de entrada igual a zero.
2) Remove o vértice da fila, insere nos visitados e diminui o grau de entrada dos filhos.
    - Se o grau de entrada do filho for igual a zero, insira na fila.

3) Repita até que a fila esteja vazia.

Tendo o passo a passo em mente, a lista de vértices visitados estará ordenado, logo basta apenas ligar as arestas correspondentes.


### Passo a passo


**Arestas:** a→b, c→b, b→d, b→f, b→i, d→h, d→j, f→h, h→i, h→j

**Graus de entrada iniciais:** a=0, c=0, b=2, d=1, f=1, i=2, h=2, j=2

---

**Início:** a e c têm grau 0, entram na fila.
- Fila: [a, c]
- Visitados: []

---

**Passo 1:** remove **a** da fila, adiciona aos visitados, decrementa grau de b (2→1), b não entra na fila.
- Fila: [c]
- Visitados: [a]

---

**Passo 2:** remove **c** da fila, adiciona aos visitados, decrementa grau de b (1→0), b entra na fila.
- Fila: [b]
- Visitados: [a, c]

---

**Passo 3:** remove **b** da fila, adiciona aos visitados, decrementa grau de d (1→0), f (1→0), i (2→1). d e f entram na fila.
- Fila: [d, f]
- Visitados: [a, c, b]

---

**Passo 4:** remove **d** da fila, adiciona aos visitados, decrementa grau de h (2→1), j (2→1). Nenhum entra na fila.
- Fila: [f]
- Visitados: [a, c, b, d]

---

**Passo 5:** remove **f** da fila, adiciona aos visitados, decrementa grau de h (1→0), h entra na fila.
- Fila: [h]
- Visitados: [a, c, b, d, f]

---

**Passo 6:** remove **h** da fila, adiciona aos visitados, decrementa grau de i (1→0) e j (1→0), ambos entram na fila.
- Fila: [i, j]
- Visitados: [a, c, b, d, f, h]

---

**Passo 7:** remove **i** da fila, adiciona aos visitados, sem filhos.
- Fila: [j]
- Visitados: [a, c, b, d, f, h, i]

---

**Passo 8:** remove **j** da fila, adiciona aos visitados, sem filhos.
- Fila: []
- Visitados: [a, c, b, d, f, h, i, j]

---


### Resultado 

<img width="989" height="292" alt="image" src="https://github.com/user-attachments/assets/753efa08-959f-4f16-a1e3-d89de746a8b3" />

>OBS: A ordem de **a** e **c** não importa, poderia começar com **a** como foi feito no passo a passo acima.




