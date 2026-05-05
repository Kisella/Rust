use graph::petgraph_visual::*;
use petgraph::{graph::{DiGraph, UnGraph}, prelude::UnGraphMap};
use petgraph::prelude::Bfs;

fn main() {
    let mut graph: petgraph::prelude::GraphMap<i32, i32, petgraph::Undirected> = UnGraphMap::from_edges(&[
        (1, 2), (3, 4), (1, 5), (2, 6), (3, 6), (3, 7), (4, 7), (4, 8), (6, 7), (7, 8)
    ]);


    generate_undirected_graphmap_dot_and_render_png(&graph, "./dot/graph.dot", "./dot/graph.png");

    // 从节点 1 开始 BFS 遍历
    let mut bfs = Bfs::new(&graph, 2);
    println!("BFS 遍历顺序（从 1 开始）：");
    while let Some(node) = bfs.next(&graph) {
        print!("{} ", node);
    }
    println!();
}