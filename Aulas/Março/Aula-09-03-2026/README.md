# Teoria dos Grafos e Computabilidade - Aula 10 

## Problema Proposto

Encontre as distâncias de $$a$$ para todos vértices de v e encontre a excentricidade de todos os vértices
bem como diâmetro e o raio do grafo.

<img width="832" height="521" alt="image" src="https://github.com/user-attachments/assets/7d6258c4-0e01-48fc-9be8-7c145776c1be" />

Tem-se novos conceitos: 

  - Excentricidade: Maior distância de um vértice $$u$$ para todos os outros vértices que pertencem a V.
  - Diâmetro: Maior excentricidade do grafo.
  - Raio: Menor excentricidade do grafo.

---

### Se G é não conexo, qual o diâmetro de G?
   Se não há caminho de $$v$$ para outros vértices, logo a excentricidade, por definição, é infinita. Logo,
o diâmetro é infinito

### Se G é direcionado, qual o diâmetro de G?
   Se não for possível traçar caminhos a partir de $$v$$, o diâmetro é infinito.

---

### Conectividade para grafos direcionados

Em um grafo não-direcionado, a conectividade está relacionada à existência de um caminho a partir de quaisquer
pares de vértices. Todavia, em se tratando de grafos direcionados, a questão é diferente.

Para que o grafo direcionado seja conexo é preciso que o seu grafo associado (grafo convertido em não direcionado)
seja conexo também.

Um grafo direcionado pode ser:

 - Fortemente conexo: Quando existe caminho para quaisquer pares de vértices.
 - Semi-fortemente conexo: Quando existe um caminho de ida ou de volta entre os pares de vértices
 - Fracamente conexo: Quando há caminho de ida e não há um de volta. 

---

## Problemas para casa 

1) Projete uma solução para determinar a conectividade de um grafo direcionado.
2) Projete uma solução para identificar um componente fortemente conexo.











