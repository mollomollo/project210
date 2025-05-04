# Project210: NYC Airbnb Neighborhood Graph

This Rust project analyzes New York City Airbnb data by building a graph of neighborhoods based on similar listing prices. It performs a breadth-first search (BFS) and calculates closeness centrality to identify the most "connected" neighborhoods.

# Dataset

- Source: NYC Airbnb Open Data on Kaggle (https://www.kaggle.com/datasets/dgomonov/new-york-city-airbnb-open-data)
- Cleaned using Python (`cleaned_airbnb.csv`)
    - Filtered listings to remove outliers (price > $1000)

# Features

- Parses CSV data into Rust structs
- Builds a graph of neighborhoods (nodes) and price-similarity (edges)
- Runs Breadth-First Search (BFS)
- Calculates Closeness Centrality for all nodes

# How to Run

with rust:

1. Clone or download the repo
2. Place `cleaned_airbnb.csv` in the root directory
3. Run the project:

to run:
cargo run
to test:
cargo test
