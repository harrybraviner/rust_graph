extern crate graph;

use graph::graph_builders::*;

#[test]
fn serialise_unconnected_graph() {
    let g = unconnected(vec![0, 1, 2, 3], false);

    let mut expected_string = String::from("// Graph\n");
    expected_string.push_str("number_of_vertices: 4\n");
    expected_string.push_str("directed: false\n");

    assert_eq!(expected_string, makeSerializationString(&g));
}

#[test]
fn serialise_undirected_graph() {
    let mut g = unconnected(vec![0, 1, 2, 3], false);
    g.add_undirected_edge(0, 3);
    g.add_undirected_edge(3, 2);
    g.add_undirected_edge(0, 1);

    let mut expected_string = String::from("// Graph\n");
    expected_string.push_str("number_of_vertices: 4\n");
    expected_string.push_str("directed: false\n");
    expected_string.push_str("0 3\n");  // This edge gets added to 0's adjacency list...
    expected_string.push_str("0 1\n");  // ...earlier than this one does.
    expected_string.push_str("2 3\n");


    assert_eq!(expected_string, makeSerializationString(&g));
}

#[test]
fn serialise_directed_graph() {
    let mut g = unconnected(vec![0, 1, 2, 3], true);
    g.add_undirected_edge(0, 3);    // Really we're adding an edge in each direction.
    g.add_directed_edge(3, 2);
    g.add_directed_edge(0, 1);

    let mut expected_string = String::from("// Graph\n");
    expected_string.push_str("number_of_vertices: 4\n");
    expected_string.push_str("directed: true\n");
    expected_string.push_str("0 3\n");  // This edge gets added to 0's adjacency list...
    expected_string.push_str("0 1\n");  // ...earlier than this one does.
    expected_string.push_str("3 0\n");
    expected_string.push_str("3 2\n");


    assert_eq!(expected_string, makeSerializationString(&g));
}
