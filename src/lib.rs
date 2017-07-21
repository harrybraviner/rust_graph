use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<T> where T : Clone + Eq + Hash {
    nodes          : Vec<T>,
    node_indices   : HashMap<T, usize>,
    node_degrees   : Vec<usize>,
    adjacency_list : Vec<Vec<usize>>,
}

impl<T> Graph<T> where T : Clone + Eq + Hash {

    pub fn new() -> Graph<T> {
        Graph { nodes : Vec::new(), node_indices : HashMap::new(), node_degrees : Vec::new(), adjacency_list : Vec::new() }
    }

    pub fn unconnected(nodes : Vec<T>) -> Graph<T> {
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

    fn number_of_vertices(&self) -> usize {
        self.nodes.len()
    }

    fn node_from_index(&self, index : usize) -> T {
        if index < self.number_of_vertices() {
            self.nodes[index].clone()
        } else {
            panic!("Called node_from_index({}) on a graph with only {} vertices!", index, self.number_of_vertices())
        }
    }

    fn index_from_node(&self, node : T) -> usize {
        match self.node_indices.get(&node) {
            Some(i) => i.clone(),
            None => panic!("Node was not present in the graph."),
        }
    }

    fn get_degree_from_index(&self, index : usize) -> usize {
        self.node_degrees[index]
    }

    fn add_directed_edge(&mut self, source_index : usize, dest_index : usize) {
        if dest_index >= self.number_of_vertices() {
            panic!("dest_index {} was >= {}, the number of vertices in the graph.", dest_index, self.number_of_vertices())
        }
        if source_index >= self.number_of_vertices() {
            panic!("source_index {} was >= {}, the number of vertices in the graph.", source_index, self.number_of_vertices())
        }
        if !self.adjacency_list[source_index].contains(&dest_index) {
            self.adjacency_list[source_index].push(dest_index);
            self.node_degrees[source_index] += 1;
        }
    }

    fn add_undirected_edge(&mut self, source_index : usize, dest_index : usize) {
        self.add_directed_edge(source_index, dest_index);
        self.add_directed_edge(dest_index, source_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_empty() {
        let g = Graph::<i32>::new();
        assert_eq!(0, g.number_of_vertices());
    }

    #[test]
    fn make_unconnected() {
        let g = Graph::unconnected(vec![1, 2, 10]);

        assert_eq!(3, g.number_of_vertices());

        assert_eq!(1, g.node_from_index(0));
        assert_eq!(2, g.node_from_index(1));
        assert_eq!(10, g.node_from_index(2));

        assert_eq!(0, g.index_from_node(1));
        assert_eq!(1, g.index_from_node(2));
        assert_eq!(2, g.index_from_node(10));

        assert_eq!(0, g.get_degree_from_index(0));
        assert_eq!(0, g.get_degree_from_index(1));
        assert_eq!(0, g.get_degree_from_index(2));
    }

    #[test]
    fn make_unconnected_and_add_edges() {
        let mut g = Graph::unconnected(vec![0, 1, 2, 3, 4, 5]);
        
        g.add_directed_edge(0, 2);
        g.add_directed_edge(4, 2);
        g.add_directed_edge(2, 3);

        assert_eq!(1, g.get_degree_from_index(0));
        assert_eq!(0, g.get_degree_from_index(1));
        assert_eq!(1, g.get_degree_from_index(2));
        assert_eq!(0, g.get_degree_from_index(3));
        assert_eq!(1, g.get_degree_from_index(4));
        assert_eq!(0, g.get_degree_from_index(5));

        // Check that adding an edge that's already in there doesn't change anything
        g.add_directed_edge(0, 2);

        assert_eq!(1, g.get_degree_from_index(0));
        assert_eq!(0, g.get_degree_from_index(1));
        assert_eq!(1, g.get_degree_from_index(2));
        assert_eq!(0, g.get_degree_from_index(3));
        assert_eq!(1, g.get_degree_from_index(4));
        assert_eq!(0, g.get_degree_from_index(5));

        g.add_undirected_edge(1, 2);

        assert_eq!(1, g.get_degree_from_index(0));
        assert_eq!(1, g.get_degree_from_index(1));
        assert_eq!(2, g.get_degree_from_index(2));
        assert_eq!(0, g.get_degree_from_index(3));
        assert_eq!(1, g.get_degree_from_index(4));
        assert_eq!(0, g.get_degree_from_index(5));
    }
}
