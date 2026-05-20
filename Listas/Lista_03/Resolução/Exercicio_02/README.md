# Exercício 02

## Problema

Dado um DAG (grafo acíclico dirigido) com arestas ponderadas, queremos encontrar o caminho de **maior custo total** partindo de um vértice fonte `s` até todos os demais vértices. Diferente do caminho mínimo, aqui maximizamos a soma dos pesos ao longo do caminho.

---

## Por que DAGs Permitem Isso com Facilidade

Em grafos gerais com ciclos, o problema do caminho mais longo é NP-difícil. Porém, a ausência de ciclos em um DAG garante que existe uma **ordem topológica** entre os vértices — uma linearização tal que toda aresta aponta de um vértice "anterior" para um "posterior". Essa propriedade é o que torna o problema tratável em tempo linear.

---

## A Ideia Central: Processamento por Ordem Topológica

A chave do algoritmo é processar os vértices exatamente na sua ordem topológica. Isso garante que, no momento em que processamos um vértice `v`, **todos os seus predecessores já foram processados** — ou seja, o caminho mais longo até cada predecessor já é conhecido e definitivo.

Isso permite que o problema seja resolvido de forma estritamente sequencial, sem revisitar vértices.

---

## O Conceito de Remoção de Base a Cada Iteração

A ideia de **remoção de base** é o coração do algoritmo e merece atenção especial.

Em cada passo, o algoritmo identifica e "remove" da consideração o vértice de menor ordem topológica entre os ainda não processados — chamamos esse vértice de **base da iteração atual**. Esse vértice tem a propriedade de que **nenhuma aresta entrante vinda de vértices não processados pode existir**, porque na ordem topológica ele precede todos os demais que restam.

Isso significa duas coisas fundamentais:

1. **Sua distância já é definitiva.** Não existe caminho ainda não explorado que possa chegar até ele — todas as contribuições possíveis já foram contabilizadas nas iterações anteriores.

2. **Ele pode ser descartado com segurança.** Após processar suas arestas saintes e atualizar as estimativas dos vizinhos, esse vértice nunca mais precisará ser revisitado. Ele é removido da "fronteira ativa" do problema.

A cada iteração, o problema efetivamente **encolhe**: o subgrafo restante é um DAG menor, com um vértice a menos, e a ordem topológica dos vértices restantes é preservada. A remoção da base não perturba a estrutura do problema — ela simplesmente o reduz.

---


## Resumo da Ideia

| Aspecto | Dijkstra | Caminho Longo em DAG |
|---|---|---|
| Ordem de processamento | menor distância estimada | ordem topológica |
| Garantia de finalização | pesos não-negativos | ausência de ciclos (DAG) |
| Remoção da base | vértice de menor custo na fila | primeiro vértice na ordem topológica |
| Funciona com pesos negativos? | não | sim |
| Complexidade | O(E log V) | O(V + E) |

A remoção de base a cada iteração não é apenas um detalhe de implementação — ela é a **garantia estrutural** de que o problema pode ser resolvido de forma gulosa e sem arrependimento, apoiada inteiramente na propriedade topológica do DAG.