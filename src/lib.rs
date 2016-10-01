pub mod mcm_bipartite;
pub mod mcm_general;
pub mod partition_bigraph;

#[cfg(test)]
mod tests {
    mod test_partition_bigraph {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        use partition_bigraph::partition_bigraph;

        #[test]
        fn two_vertexes_one_edge() {
            let graph: Vec<Vec<u8>> = vec![vec![1], vec![0]];
            let result = partition_bigraph(graph).unwrap();
            let expected: [Vec<u8>; 2] = [vec![1], vec![0]];
            asert_partitions_match(expected, result);
        }

        fn vec_to_set(vec: Vec<u8>) -> HashSet<u8> {
            let mut victim = vec.clone();
            let x: HashSet<u8> = victim.drain(..).collect(); // dedup
            return x;
        }

        fn asert_partitions_match(expected: [Vec<u8>; 2], actual: [Vec<u8>; 2]) {
            let e0 = vec_to_set(expected[0].clone());
            let e1 = vec_to_set(expected[1].clone());
            let a0 = vec_to_set(actual[0].clone());
            let a1 = vec_to_set(actual[1].clone());
            assert!(a0 == e0 || a0 == e1 && a1 == e1 || a1 == e0);
        }
    }

    mod test_mcm_bipartite {
        use mcm_bipartite::match_mcm_bipartite;

        #[test]
        fn empty_graph_empty_match() {
            let graph: Vec<Vec<u8>> = vec![];
            let expected: Vec<[u8; 2]> = vec![];
            assert_eq!(expected, match_mcm_bipartite(graph));
        }

        #[test]
        fn single_vertex_empty_match() {
            let graph: Vec<Vec<u8>> = vec![vec![]];
            let expected: Vec<[u8; 2]> = vec![];
            assert_eq!(expected, match_mcm_bipartite(graph));
        }

        #[test]
        fn two_vertexes_one_edge_one_match() {
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
