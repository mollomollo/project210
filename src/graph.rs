// graph.rs
// Defines Node, Edge, and Graph types. Implements BFS and closeness centrality.
use std::collections::VecDeque;

// A neighborhood node in the graph, containing name and average Airbnb price.
#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub avg_price: f32,
}
// An edge between two neighborhoods, by their index in the graph's node list.
#[derive(Debug, Clone)]
pub struct Edge {
    pub source: usize,  // index of node in the graph
    pub target: usize,  // index of node in the graph
}
// The whole graph
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    // A new empty graph
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // Add a node to the graph
    pub fn add_node(&mut self, name: String, avg_price: f32) {
        let node = Node { name, avg_price };
        self.nodes.push(node);
    }

    // Add an edge between two nodes (by their indices)
    pub fn add_edge(&mut self, source: usize, target: usize) {
        let edge = Edge { source, target };
        self.edges.push(edge);
    }

    // Performs a Breadth-First Search starting from the given node index. Uses a queue (VecDeque) to traverse the graph layer-by-layer.
    // Input: `start` - Index of the starting node in the nodes vector
    //Output: Prints the names of all visited neighborhoods
    pub fn bfs(&self, start: usize) { 
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        println!("Starting BFS from: {}", self.nodes[start].name);

        while let Some(current) = queue.pop_front() {
            println!("Visited: {}", self.nodes[current].name);

            // Find neighbors
            for edge in &self.edges {
                let neighbor = if edge.source == current {
                    Some(edge.target)
                } else if edge.target == current {
                    Some(edge.source)
                } else {
                    None
                };

                if let Some(neigh_idx) = neighbor {
                    if !visited[neigh_idx] {
                        visited[neigh_idx] = true;
                        queue.push_back(neigh_idx);
                    }
                }
            }
        }
    }

    // Computes closeness centrality for each neighborhood. For each node, run BFS to compute total distance to all others.
    // Centrality = 1 / total_distance (or 0 if unreachable)
    // outputs vector of neighborhood name and centrality score, where higher = more central.
pub fn closeness_centrality(&self) -> Vec<(String, f32)> {
    let mut centralities = Vec::new();

    for start in 0..self.nodes.len() {
        let mut visited = vec![false; self.nodes.len()]; // tracks visited nodes
        let mut distance = vec![0; self.nodes.len()];
        let mut queue = VecDeque::new(); // BFS queue

        visited[start] = true;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() { // BFS loop to calculate distances from this node
            for edge in &self.edges {
                let neighbor = if edge.source == current {
                    Some(edge.target)
                } else if edge.target == current {
                    Some(edge.source)
                } else {
                    None
                };

                if let Some(neigh_idx) = neighbor {
                    if !visited[neigh_idx] {
                        visited[neigh_idx] = true;
                        distance[neigh_idx] = distance[current] + 1;  // update neighbor's distance
                        queue.push_back(neigh_idx);
                    }
                }
            }
        }

        let total_distance: u32 = distance.iter().sum();
        let centrality = if total_distance > 0 {
            1.0 / total_distance as f32
        } else {
            0.0
        };

        centralities.push((self.nodes[start].name.clone(), centrality));
    }

    centralities
}

}



