extern crate graph;
use graph::*;

#[test]
fn breadth_first_traversal_node_processing() {
    let g = graph_builders::from_file("test_data/graph2").unwrap();

    let mut count = 0;
    g.breadth_first_iter_from_index(|_| { count += 1; }, |_, _| { }, 0);
    assert_eq!(3, count);
}

#[test]
fn breadth_first_traversal_edge_processing() {
    let g = graph_builders::from_file("test_data/graph2").unwrap();

    let mut edges = Vec::<(usize, usize)>::new();
    g.breadth_first_iter_from_index(|_| { }, |s, d| { edges.push((*s, *d)); }, 0);

    assert_eq!(4, edges.len());
    assert_eq!((0, 1), edges[0]);
    assert_eq!((0, 2), edges[1]);
    assert_eq!((1, 0), edges[2]);
    assert_eq!((2, 0), edges[3]);
}
