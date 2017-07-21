use super::Graph;
use std::collections::HashMap;
use std::hash::Hash;

pub fn unconnected<T : Eq + Clone + Hash>(nodes : Vec<T>) -> Graph<T> {
    let hash_map : HashMap<T, usize> =
        nodes.iter()
             .cloned()
             .enumerate()
             .map(|(i, x)| { (x, i) })
             .collect();
    let node_degrees = vec![0; nodes.len()];
    let adjacency_list = vec![Vec::new(); nodes.len()];
    Graph { nodes : nodes.clone(), node_indices : hash_map, node_degrees : node_degrees, adjacency_list : adjacency_list }
}

fn foo () {
    print!("yo");
}
