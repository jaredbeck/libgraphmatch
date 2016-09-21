pub mod mcm_bipartite;
pub mod mcm_general;

#[cfg(test)]
mod tests {
    mod test_mcm_bipartite {
        use mcm_bipartite::match_mcm_bipartite;

        #[test]
        fn empty_graph_empty_match() {
            println!("empty_graph_empty_match");
            let graph: Vec<Vec<u8>> = vec![];
            let expected: Vec<[u8; 2]> = vec![];
            assert_eq!(expected, match_mcm_bipartite(graph));
        }

        #[test]
        fn single_vertex_empty_match() {
            println!("single_vertex_empty_match");
            let graph: Vec<Vec<u8>> = vec![vec![]];
            let expected: Vec<[u8; 2]> = vec![];
            assert_eq!(expected, match_mcm_bipartite(graph));
        }

        #[test]
        fn two_vertexes_one_edge_one_match() {
            println!("two_vertexes_one_edge_one_match");
            let graph: Vec<Vec<u8>> = vec![vec![1], vec![0]];
            let expected: Vec<[u8; 2]> = vec![[0, 1]];
            assert_eq!(expected, match_mcm_bipartite(graph));
        }
    }

    mod test_mcm_general {
        use mcm_general::match_mcm_general;

        #[test]
        fn test_mcm_general() {
            assert_eq!("mcm general", match_mcm_general());
        }
    }
}
