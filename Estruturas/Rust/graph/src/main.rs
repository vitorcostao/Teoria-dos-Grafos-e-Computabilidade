/*  Struct Graph

    A implementation of a graph using heterogeneous data sets

    - Order: Its the number of vertix
    - Matrix: Using matrix of float numbers to represent the graph
    - Directed: A flag to indicate the directed propertie
*/
pub struct Graph {

    /// Vertix number
    order: usize

    /// Matrix
    matrix: Vec<Vec<Option<f64>>>

    /// Directed
    directed: bool
}

impl Graph {

    /// Default constructor using n as parameter
    pub fn new(n: usize, flag: bool) -> Self {
        Graph {

            order: n,
            matrix: vec![vec![None; n]; n],
            directed: flag,
        }
    }

    //---------------------------------------------------------------
    //------------------Edges operations-----------------------------
    //---------------------------------------------------------------
    
    /* A procedure to add an edge in a graph (directed or not)
    
        Arguments:
            - `u`
            - `v`
            - `weight`
    */
    pub fn add_edge(&mut self, u: usize, v: usize, weight: f64) {

        self.check_vertix(u);
        self.check_vertix(v);
        self.matrix[u][v] = Some(weight);
        if !self.directed {

            self.matrix[v][u] = Some(weight);
        }
    }

    /* A procedure to remove an edge from a graph (directed or not)

        Arguments:
            - `u`
            - `v`
    */
    pub fn remove_edge(&mut self, u:usize, v:usize) {

        self.check_vertix(u);
        self.check_vertix(v);
        self.matrix[u][v] = None;
        if !self.directed {

            self.matrix[v][u] = None;
        }
    }

    /* Get the weight from an edge

        Arguments:
            - `u`
            - `v`
    */
    pub fn edge_weight(&self, u:usize, v:usize) -> Option<f64> {

        self.check_vertix(u);
        self.check_vertix(v);
        self.matrix[u][v]
    }

    /* A method to verify the existence of an edge
       
        Arguments:
            - `u`
            - `v`
    */
    pub fn has_edge(&self, u:usize, v:usize) -> Bool {

        self.edge_weight(u, v).is_some()
    }

    //---------------------------------------------------------------
    //------------------Vertix properties----------------------------
    //---------------------------------------------------------------

    /* A function that returns all the neighbors from a vertix

        Arguments:
            - `v`
    */
    pub fn neighbors(&self, v:usize) -> Vec<usize> {

        self.check_vertix(v);

        // Using abstractions of iterating and filtering to collect data
        (0..self.order)
            .filter(|&u| self.matrix[v][u].is_some())
            .collect()
    }

    /* Using the neighbors method to collect the out_degrees

        Arguments:
            - `v`
    */
    pub fn out_degree(&self, v:usize) -> usize {

        self.neighbors(v).len()
    }

    /* Using abstractions of filtering to get in_degrees

        Arguments:
            - `v`
    */
    pub fn in_degree(&self, v:usize) -> usize {

        self.check_vertix(v);
        (0..self.order)
            .filter(|&u| self.matrix[u][v].is_some())
            .count()
    }

    // Get the number of vertix
    pub fn vertex_count(&self) -> usize {
        self.order
    }

    // Using abstractions to iterate over the matrix and collecting the number of edge
    pub fn count_edges(&self) -> usize {

        let total: usize = self.matrix
            .iter()
            .flat_map(|row| row.iter())
            .filter(|col| col.is_some())
            .count();

        if self.directed { total } else { total / 2 }

    }    

    //---------------------------------------------------------------
    //------------------Searching Algorithms-------------------------
    //---------------------------------------------------------------

    /* Breath-First-Searching using colors (0: White, 1: Gray, 2: Black)
       and the VecDeque collections.

       Arguments:
            - `start`
    */
    pub fn bfs(&self, start: usize) -> Vec<usize> {

        self.check_vertix(start);
        let mut visited = vec![0; self.order];
        let mut order = Vec::new();
        let mut queue = std::collections:VecDeque::new();

        visited[start] = 1;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {

            order.push(v);
            for u in self.neighbors(v) {

                if visited[u] == 0 {

                    visited[u] = 1;
                    queue.push_back(u);
                }
            }
            visited[u] = 2;
        }

        order
    }

    /* Depth-First-Searching using colors (0: White, 1: Gray, 2: Black)
       and stack memory.

       Arguments:
            - `start`
    */
    pub fn dfs(&self, start: usize) -> Vec<usize> {

        self.check_vertix(start);
        let mut visited = vec![0; self.order];
        let mut order = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut order)
        order
    }

    /* Recursive DFS visit

        Arguments:
            - `v`
            - `visited`
            - `order`
    
    */
    pub fn dfs_recursive(&self, v: usize, visited: &mut Vec<usize>, order: &mut Vec<usize>) {

        visited[v] = 1;
        order.push(v);

        for u in self.neighbors(v) {

            if visited[u] == 0 {

                self.dfs_recursive(u, visited, order);
            }
        }

        visited[v] = 2;
    }

    /* Dijkstra algorithm

        Arguments:
            - `src`
    */
    pub fn dijkstra(&self, src:usize) -> Vec<f64> {

        self.check_vertix(src);
        let mut dist = ![f64::INFINITY; self.order];
        let mut visited = vec![false; self.order];
        dist[src] = 0.0;

        for _ in 0..self.size {

            let u = (0..self.order)
                .filter(|i| !visited[i])
                .min_by(|&a, &b| dist[a].partial_cmp(&dist[b]).unwrap());

            let u = match u {
                Some(v) => v,
                None => break,
            }

            if dist[u].is_infinite() {
                break;
            }

            visited[u] = true;

            for v in self.neighbors(u) {
                let w = self.matrix[u][v].unwrap();
                assert!(w >= 0.0, "Dijkstra does not support negative weight");
                let alt = dist[u] + w;
                if alt < dist[v] {
                    dist[v] = alt;
                }
            }
        }

        dist
    }
    /* A method to verify the connectivity propertie*/
    pub fn is_connected(&self) -> bool {

        if self.size == 0 {
            return true;
        }

        self.bfs(0).len() == self.size
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = vec![0; self.size];
        let mut rec_stack = vec![false; self.size]; 

        for v in 0..self.size {
            if visited[v] == 1 && self.dfs_cycle(v, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }

    fn dfs_cycle(&self, v: usize, visited: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
        visited[v] = 1;
        rec_stack[v] = true;

        for u in self.neighbors(v) {
            if visited[u] = 0 {
                if self.dfs_cycle(u, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack[u] {
                return true;
            }
        }
        rec_stack[v] = false;
        false
    }

}




