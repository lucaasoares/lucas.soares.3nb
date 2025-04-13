
use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.adjacency_list.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.adjacency_list.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }

    fn dfs(&self, start: &str) {
        let mut visited = HashSet::new();
        println!("DFS");
        self.dfs_recursive(start, &mut visited);
    }

    fn dfs_recursive(&self, vertex: &str, visited: &mut HashSet<String>) {
        if visited.contains(vertex) {
            return;
        }
        println!("{}", vertex);
        visited.insert(vertex.to_string());

        if let Some(neighbors) = self.adjacency_list.get(vertex) {
            for neighbor in neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }

    fn bfs (&self, start: &str){
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());
        
        println!("BFS");

        while let Some(current) = queue.pop_front() {
            println!("{}", current);

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.to_string());
                        queue.push_back(neighbor.to_string());
                    }
                }
            }
        }

    }    
        
}

fn main() {
    let mut graph = Graph::new();

    graph.add_vertex("A");
    graph.add_vertex("B");
    graph.add_vertex("C");
    graph.add_vertex("D");
    graph.add_vertex("E");
    graph.add_vertex("F");

    graph.add_edge("A", "B");
    graph.add_edge("B", "F");
    graph.add_edge("F", "E");
    graph.add_edge("C", "B");
    graph.add_edge("A", "F");
    graph.add_edge("B", "A");
    graph.add_edge("B", "D");

    // Executa DFS
    graph.dfs("A");
    println!("---");
    // Executa BFS
    graph.bfs("A");
}