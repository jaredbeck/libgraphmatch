extern crate std;

/*
Separates graph's vertices into two disjoint sets so that every edge of the
graph connects vertices from different sets. If it's possible, the graph is
bipartite.
*/
pub fn partition_bigraph(graph: Vec<Vec<u8>>) -> Option<[Vec<u8>; 2]> {
    println!("partition_bigraph graph size is: {}", graph.len());

    // Begin a breadth-first search (BFS)
    let mut pending: Vec<usize> = vec![];

    // each node is given the opposite color to its parent in the search tree
    // All vertexes begin uncolored (0). White is 1, black is 2.
    let mut colors: Vec<u8> = std::iter::repeat(0).take(graph.len()).collect::<Vec<_>>();

    // Start the BFS from each vertex to make sure
    // that all connected components of the graph are processed.
    for i in 0..graph.len() {
        println!("i: {}", i);

        if colors[i] == 0 {
            colors[i] = 1;
            println!("color i as white");
            pending.push(i);
        }
        while pending.len() > 0 {
            let cur = pending.pop().unwrap();
            println!("cur: {}", cur);

            let cur_color = colors[cur as usize];
            let next_color = if cur_color == 1 { 2 } else { 1 };
            for neighbor in graph[cur as usize].iter() {
                println!("neighbor: {}", neighbor);

                let neighbor_color = colors[*neighbor as usize];
                if neighbor_color == next_color {
                    // noop
                } else if neighbor_color == cur_color {
                    println!("neighbor_color == cur_color, at neighbor: {}", neighbor);
                    return None; // odd-cycle found, the graph is not bipartite
                } else if neighbor_color == 0 {
                    colors[*neighbor as usize] = next_color;
                    println!("color neighbor {} as color: {}", neighbor, next_color);
                    pending.push(*neighbor as usize);
                } else {
                    panic!("invalid color");
                }
            }
        }
    }

    println!("Colors: {:?}", colors);

    let mut white = vec![];
    let mut black = vec![];
    for (i, color) in colors.iter().enumerate() {
        if *color == 1 { white.push(i as u8); }
            else if *color == 2 { black.push(i as u8); }
                else { panic!("invalid color"); }
    }

    // Return the 2-coloring found. The graph is bipartite.
    return Some([white, black]);
}
