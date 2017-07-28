extern crate graph;
use graph::*;

#[test]
fn make_empty() {
    let g = Graph::<i32>::new();
    assert_eq!(0, g.number_of_vertices());
}

#[test]
fn make_unconnected() {
    let g = graph_builders::unconnected(vec![1, 2, 10], false);

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
    let mut g = graph_builders::unconnected(vec![0, 1, 2, 3, 4, 5], false);
    
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

#[test]
fn read_graph_from_file() {
    let g = graph_builders::from_file("test_data/graph1").unwrap();

    assert_eq!(3, g.number_of_vertices());

    assert_eq!(1, g.get_degree_from_index(0));
    assert_eq!(1, g.get_degree_from_index(1));
    assert_eq!(1, g.get_degree_from_index(2));

    assert_eq!(0, g.index_from_node(0));
    assert_eq!(1, g.index_from_node(1));
    assert_eq!(2, g.index_from_node(2));
}

#[test]
fn read_undirected_graph_from_file() {
    let g = graph_builders::from_file("test_data/graph2").unwrap();

    assert_eq!(3, g.number_of_vertices());

    assert_eq!(2, g.get_degree_from_index(0));
    assert_eq!(1, g.get_degree_from_index(1));
    assert_eq!(1, g.get_degree_from_index(2));

    assert_eq!(0, g.index_from_node(0));
    assert_eq!(1, g.index_from_node(1));
    assert_eq!(2, g.index_from_node(2));
}
