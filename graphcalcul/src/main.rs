use petgraph::graph::Graph;
use petgraph::algo::dijkstra;


fn main() {
    let mut new_graph: Graph<&str, i32> = Graph::new();

    let a = new_graph.add_node("A");
    let b = new_graph.add_node("B");
    let c = new_graph.add_node("C");
    let d = new_graph.add_node("D");

    // Add edges with weights
    new_graph.add_edge(a, b, 4);
    new_graph.add_edge(a, c, 2);
    new_graph.add_edge(c, b, 1);
    new_graph.add_edge(c, d, 5);
    new_graph.add_edge(b, d, 3);

    let distance = dijkstra(&new_graph, a, None, |e: petgraph::graph::EdgeReference<i32>| *e.weight());
    println!("distance A->B = {:?}", distance.get(&b));
}
