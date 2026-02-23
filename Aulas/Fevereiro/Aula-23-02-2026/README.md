# Teoria dos Grafos e Computabilidade - Aula 04

## Análise de representação de grafos

### 1) Matriz

  | 0 | 0 | 0 | 0 |
  |---|---|---|---|
  | 1 | 1 | 0 | 1 |
  | 0 | 0 | 1 | 0 |
  | 1 | 1 | 1 | 0 |
  
  Um grafo simples e direcionado pode ser representado por uma matriz. No caso acima, as linhas e colunas representam os vértices enquanto
  os valores representam as arestas do grafo, logo, o vértice 1 possui arestas apontando para os vértices 0, 1 e 3. O vértice 2 possui uma aresta
  apontando para ele mesmo. E o vértice 3 possui arestas apontando para os vértices 0, 1, e 2;

  O problema dessa representação é que para adicionar vértices, a matriz precisaria mudar suas dimensões. Além disso, não é possível representar
  arestas com pesos em conjunto com arestas paralelas, pois com os pesos seria necessário números tal qual as arestas paralelas.

### 2) Listas

  | Vértices | Arestas        |
  |----------| -------------- |
  | 0        |   //           |
  | 1        | -> 0 -> 1 -> 3 |
  | 2        | -> 2           | 
  | 3        | -> 0 -> 1 -> 2 |

  Um grafo simples e direcionado pode ser representado por uma lista. No caso acima, cada elemento dessa lista seria um nó inicial de uma lista
  ligada, o qual possuiria as aresta sendo as ligações.

  O problema dessa representação não está relacionado com o fato de adicionar vértices, porém inserir arestas seria complexo poderia causar uma 
  refatoração da estrutura e até mesmo uma quebra dela.

### Representação gráfica

  <img width="300" height="300" alt="image" src="https://github.com/user-attachments/assets/b9a43163-1bc2-42a9-97bb-cc8c6542a649" />


---

## Algoritmo: Busca em Profundidade (DFS)


Uma das alternativas para calcular o número de componentes conexos de um grafo é a busca em profundidade (DFS), tem-se o pseudocódigo deste algoritmo:

    Algoritmo DFS_Com_Cores(grafo)
        Para cada vertice V em grafo.V faça:
            cor[V] = BRANCO
        Fim Para
    
        contador_componentes = 0
    
        Para cada vertice V em grafo.V faça:
            Se cor[V] == BRANCO então:
                contador_componentes = contador_componentes + 1
                DFS_Visita(V)
            Fim Se
        Fim Para
    Fim Algoritmo
    
    Procedimento DFS_Visita(U)
        cor[U] = CINZA  // Começou a ser explorado
        Exibir(U)
    
        Para cada vizinho V em grafo.adjacentes(U) faça:
            Se cor[V] == BRANCO então:
                DFS_Visita(V)
            Senão se cor[V] == CINZA então:
                // Detectou um ciclo!
            Fim Se
        Fim Para
    
        cor[U] = PRETO  // Finalizado
    Fim Procedimento

---


## Definição de subgrafo induzido

Um subgrafo induzido contém um subconjunto de vértices do grafo original e todas as arestas que conectam esses mesmos vértices no grafo principal.

