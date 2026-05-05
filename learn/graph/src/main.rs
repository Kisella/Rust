use graph::petgraph_visual::generate_dot_and_render_png;
use petgraph::graph::DiGraph;
use petgraph::prelude::Bfs;
fn main() {
    let mut graph = DiGraph::new();
    let a = graph.add_node("Node A");
    let b = graph.add_node("Node B");
    graph.add_edge(a, b, 43);

    generate_dot_and_render_png(&graph, "./dot/graph.dot", "./dot/graph.png");


    let mut bfs = Bfs::new(&graph, a);
    while let Some(nx) = bfs.next(&graph) {
        println!("Visited: {:?}", graph[nx]);
    }
}