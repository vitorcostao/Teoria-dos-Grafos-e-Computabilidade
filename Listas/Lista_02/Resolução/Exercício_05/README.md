# Teoria dos Grafos e Computabulidade - Lista 01

## Exercício 05

Seja G = (V, E) um grafo direcionado. Sabemos, por definição, que para cada par de vértices u, v ∈ V em um
componente fortemente conexo de um grafo, temos um caminho de u para v bem como um caminho v para u.
Seja e ∈ E uma aresta qualquer de um componente fortemente conexo. Após remover a aresta e, ainda existirá
um caminho de u para v ou de v para u?

### Resolução

Tendo as definições em mãos, a resposta é sim: ainda existirá um caminho de u para v ou v para u. A condição
final do enunciado expressa um grafo semi-fortemente conexo e nesse contexto todo grafo fortemente conexo é também
semi-fortemente conexo. Com isso, a remoção de APENAS uma aresta não irá fazer com que o grafo se torne fracamente
conexo. Portanto, a afirmação é verdadeira.