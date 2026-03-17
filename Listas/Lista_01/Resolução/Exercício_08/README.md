# Teoria dos Grafos e Computabilidade - Lista 01

![alt text](./images/image.png)

---

### Resolução 01

Para modelar o problema, cada vértice representa um prédio e cada aresta representa o caminho
de um prédio para outro utilizando os pesos para ser a distância entre os prédios. 
Desse modo, para encontrar a menor distância entre o prédio 34 até o
Teatro João Paulo II será feito o seguinte algoritmo:

```
distancia_min(grafo, predio_34, teatro_JP) -> Retorna Caminho

    para todo v e V
        d[v] = infinito
        ant[v] = nulo
    fim para

    d[predio_34] = 0
    f.insere(predio_34, prioridade = d[predio_34])

    enquanto !f.vazio()
        u = f.remove_min()

        se d[u] = prioridade_removida 
            para todo u* e N[u]
                se d[u*] > d[u] + peso(u, u*)
                    d[u*] = d[u] + peso(u, u*)
                    ant[u*] = u
                    F.insere(u*, prioridade = d[u*])
                fim se
            fim para
        fim se
    fim enquanto

    u = teatro_JP
    pilha.insere(u)

    enquanto ant[u] != nulo 
        pilha.insere(ant[u])
        u = ant[u]
    fim enquanto
    
    retorna pilha
fim algoritmo
```

---

### Resolição 02

---

### Resolição 03

Para solucionar o problema, basta separar os vértices em conjuntos de acordo com a inicial dos nomes dos alunos e realizar
combinações tomadas dois a dois para obter os pares possíveis.