# Teoria dos Grafos e Computabilidade - Aula 20

## Exercícios

### 1) Ache o MST para os grafos a seguir utilizando as três abordagens

| ![](https://github.com/user-attachments/assets/c6b9ff8e-3111-44a3-89e6-3c04c0d560de) | ![](https://github.com/user-attachments/assets/1344b1e4-bdd2-486a-ab2e-43f8082c7717) | ![](https://github.com/user-attachments/assets/17eb08e9-1079-449e-af80-0e90be8ce965) |
|---|---|---|

#### Grafo 1 — Prim (partindo de a)

| Passo     | Vértice adicionado | Aresta escolhida | Soma  |
|-----------|--------------------|------------------|-------|
| 1         | a                  | —                | 0     |
| 2         | f                  | a-f              | 2     |
| 3         | d                  | f-d              | 1     |
| 4         | c                  | f-c              | 1     |
| 5         | e                  | c-e              | 2     |
| 6         | b                  | a-b              | 3     |
| **Total** |                    |                  | **9** |

#### Grafo 2 — Kruskal

| Passo     | Aresta | Peso | Ação              | Componentes            |
|-----------|--------|------|-------------------|------------------------|
| 1         | a-b    | 1    | Adiciona          | {a,b}, {c}, {d}, {e}   |
| 2         | b-c    | 1    | Adiciona          | {a,b,c}, {d}, {e}      |
| 3         | d-b    | 1    | Adiciona          | {a,b,c,d}, {e}         |
| 4         | a-c    | 1    | Ciclo — ignora    | {a,b,c,d}, {e}         |
| 5         | a-d    | 2    | Ciclo — ignora    | {a,b,c,d}, {e}         |
| 6         | d-e    | 2    | Adiciona          | {a,b,c,d,e}            |
| **Total** |        | **5**|                   |                        |

#### Grafo 3 — Reverse-Delete

| Passo     | Aresta | Peso  | Ação    | Motivo                  |
|-----------|--------|-------|---------|-------------------------|
| 1         | d-e    | 7     | Remove  | Grafo continua conexo   |
| 2         | a-d    | 5     | Remove  | Grafo continua conexo   |
| 3         | d-c    | 3     | Mantém  | Remoção desconecta d    |
| 4         | a-b    | 1     | Mantém  | Remoção desconecta b    |
| 5         | b-e    | 1     | Mantém  | Remoção desconecta e    |
| 6         | c-e    | 1     | Mantém  | Remoção desconecta e    |
| 7         | c-b    | 1     | Remove  | Grafo continua conexo   |
| 8         | a-c    | 1     | Mantém  | Remoção desconecta a    |
| **Total** |        | **7** |         |                         |