# Exercício 01

## Problema

Dado um dígrafo com arestas ponderadas, queremos encontrar o caminho de menor custo partindo de um vértice fonte `s` até todos os demais vértices, com a restrição de que os pesos das arestas ao longo do caminho sejam **estritamente crescentes** ou **estritamente decrescentes**. Um caminho com essa propriedade é chamado de **monotônico**.

---

## A Ideia Central: Relaxamento com Estado

O algoritmo de Dijkstra clássico mantém, para cada vértice, uma única informação: a menor distância conhecida até ele. Para o problema monotônico, isso não é suficiente.

O motivo é que **o que importa não é apenas onde você está, mas também como você chegou lá** — mais especificamente, qual foi o peso da última aresta que você usou. Isso porque a validade de qualquer próxima aresta depende do peso da aresta anterior.

A solução é expandir o conceito de "estado".

---

## O Conceito de Estado

No algoritmo clássico, um estado é simplesmente um vértice `v`.

No algoritmo monotônico, um estado é o par:

> **(v, w_última)**

onde `v` é o vértice atual e `w_última` é o peso da última aresta percorrida para chegar em `v`.

Dois caminhos que chegam ao mesmo vértice `v` com **pesos de última aresta diferentes** são estados distintos, pois eles permitem continuar por arestas diferentes.

### Por que isso resolve o problema?

Ao manter esse par, conseguimos decidir localmente se uma próxima aresta `(v → u)` com peso `w` pode ser usada:

- Em um caminho **crescente**: só se `w > w_última`
- Em um **decrescente**: só se `w < w_última`

Sem essa informação no estado, seria impossível fazer essa verificação sem reconstruir o caminho inteiro.

---

## Dois Subproblemas Independentes

Como a monotonicidade pode ser crescente **ou** decrescente, o problema se divide naturalmente em dois subproblemas independentes:

1. **Caminho mínimo monotônico crescente** partindo de `s`
2. **Caminho mínimo monotônico decrescente** partindo de `s`

Cada subproblema é resolvido separadamente, e o resultado final para cada vértice é o menor custo entre as duas soluções.

---

## A Estrutura do Espaço de Estados

Podemos pensar no espaço de estados como um **grafo auxiliar implícito**, onde cada nó é um par `(v, w_última)`. As arestas nesse grafo auxiliar representam transições válidas: sair do estado `(v, w_última)` pela aresta `(v → u, w)` leva ao estado `(u, w)`, desde que `w` respeite a restrição de monotonicidade.

Esse espaço de estados é potencialmente muito maior que o grafo original, pois um mesmo vértice `v` pode aparecer em vários estados distintos — um para cada peso de aresta com que pode ser atingido.

---

## A Garantia de Correção

A estrutura de estados garante que:

- Nunca usamos uma aresta que viole a monotonicidade, pois a restrição é verificada localmente no estado.
- Não há problema em visitar o mesmo vértice `v` múltiplas vezes, desde que cada visita corresponda a um estado `(v, w_última)` distinto — cada um pode abrir caminhos futuros diferentes.
- A otimalidade é preservada porque, dentro de cada classe de estados (mesmo vértice, mesmo peso de última aresta), basta manter o caminho de menor custo total — o comportamento futuro possível é idêntico para todos os caminhos nessa classe.
