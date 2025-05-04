// tests.rs
// Basic tests for graph construction and dummy BFS on a small test graph

use crate::graph::Graph;

#[test]
fn test_graph_construction() {
    let mut graph = Graph::new();
    graph.add_node("A".to_string(), 100.0);
    graph.add_node("B".to_string(), 105.0);
    graph.add_edge(0, 1);

    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 1);
    assert_eq!(graph.edges[0].source, 0);
    assert_eq!(graph.edges[0].target, 1);
}
