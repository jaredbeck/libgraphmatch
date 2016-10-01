extern crate std;
use partition_bigraph::partition_bigraph;

// Maximum Cardinality Matching in Bipartite Graphs
//
// Takes `graph`, a vector where each element represents a vertex. The value of
// each element is a vector where each element represents an edge. For example
// `vec![vec![1], vec![0]]` is a graph with two vertexes and one edge.
pub fn match_mcm_bipartite(graph: Vec<Vec<u8>>) -> Vec<[u8; 2]> {
    println!("match_mcm_bipartite graph size is: {}", graph.len());
    let result = partition_bigraph(graph);
    match result {
        Some(partition) => {
            let u = partition[0].clone();
            println!("u: {:?}", u);
            let v = partition[1].clone();
            println!("v: {:?}", v);
        }
        None => {
            panic!("graph is not bipartite");
        }
    }
    return vec![];
}
