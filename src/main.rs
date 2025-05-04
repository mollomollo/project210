// main.rs
// Loads data, builds the Airbnb graph, runs BFS and centrality, prints results.

mod data;
mod graph;

use data::load_listings;
use graph::Graph;
use std::collections::HashMap;


fn main() {
    let filepath = "cleaned_airbnb.csv"; // Load CSV

    match load_listings(filepath) {
        Ok(listings) => {
            println!("Successfully loaded {} listings.", listings.len());
            println!("Example listing: {:?}", listings[0]);

            let mut neighborhood_prices: HashMap<String, Vec<u32>> = HashMap::new(); // Group listings by neighborhood to compute average price

            for listing in listings {
                neighborhood_prices
                    .entry(listing.neighbourhood.clone())
                    .or_insert(Vec::new())
                    .push(listing.price);
            }

            let mut graph = Graph::new();

            // Step 2: Add nodes
            for (neigh, prices) in &neighborhood_prices {
                let sum: u32 = prices.iter().sum();
                let avg = sum as f32 / prices.len() as f32;
                graph.add_node(neigh.clone(), avg);
            }

            // Step 3: Add edges between similar neighborhoods
            let threshold = 20.0; // If average prices differ by <= 20 dollars, connect them

            for i in 0..graph.nodes.len() {
                for j in (i + 1)..graph.nodes.len() {
                    let price_diff = (graph.nodes[i].avg_price - graph.nodes[j].avg_price).abs();
                    if price_diff <= threshold {
                        graph.add_edge(i, j);
                    }
                }
            }

            println!("Graph built with {} nodes and {} edges!", graph.nodes.len(), graph.edges.len());

            // Print first 5 nodes to check
            for node in graph.nodes.iter().take(5) {
                println!("{:?}", node);
            }

            // Print first 5 edges to check
            for edge in graph.edges.iter().take(5) {
                println!("{:?}", edge);
            }

            graph.bfs(0); // Run BFS from the first node to confirm connectivity
            // Calculate and print closeness centrality
let centralities = graph.closeness_centrality();

// Sort by centrality highest to lowest
let mut sorted = centralities.clone();
sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

println!("Top 5 neighborhoods by closeness centrality:");
for (name, score) in sorted.iter().take(5) {
    println!("{} -> {}", name, score);
}



        },
        Err(e) => {
            eprintln!("Error loading listings: {}", e);
        }
    }
}
#[cfg(test)]
mod tests;

