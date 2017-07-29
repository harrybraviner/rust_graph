extern crate graph;
use graph::graph_builders;

#[test]
fn traversal_order() {
    let g = graph_builders::from_file("test_data/graph1").unwrap();

    let mut discovery_order = Vec::<usize>::new();
    let mut processed_order = Vec::<usize>::new();

    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _, _| { },
                                  0);
    assert_eq!(vec![0, 1], discovery_order);
    assert_eq!(vec![1, 0], processed_order);

    discovery_order.clear();
    processed_order.clear();
    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _, _| { },
                                  1);
    assert_eq!(vec![1, 0], discovery_order);
    assert_eq!(vec![0, 1], processed_order);

    discovery_order.clear();
    processed_order.clear();
    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _, _| { },
                                  2);
    assert_eq!(vec![2, 0, 1], discovery_order);
    assert_eq!(vec![1, 0, 2], processed_order);
}

#[test]
fn traversal_order_2() {
    let g = graph_builders::from_file("test_data/graph3").unwrap();

    let mut discovery_order = Vec::<usize>::new();
    let mut processed_order = Vec::<usize>::new();

    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _, _| { },
                                  0);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], discovery_order);
    assert_eq!(vec![4, 3, 2, 1, 5, 0], processed_order);

    discovery_order.clear();
    processed_order.clear();
    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _, _| { },
                                  2);
    assert_eq!(vec![2, 1, 0, 3, 4, 5], discovery_order);
    assert_eq!(vec![4, 3, 5, 0, 1, 2], processed_order);
}

#[test]
fn edge_processing_directed() {
    let g = graph_builders::from_file("test_data/graph4").unwrap();

    let mut edges = Vec::<(usize, usize, graph::DFSEdgeType)>::new();

    g.depth_first_iter_from_index(|_| { },
                                  |_| { },
                                  |s, d, t| edges.push((*s, *d, t)),
                                  2);
    let expected_edges =
        vec![(2, 0, graph::DFSEdgeType::Tree),
             (0, 4, graph::DFSEdgeType::Tree),
             (4, 0, graph::DFSEdgeType::Back),
             (4, 2, graph::DFSEdgeType::Back),
             (4, 5, graph::DFSEdgeType::Tree),
             (4, 6, graph::DFSEdgeType::Tree),
             (2, 1, graph::DFSEdgeType::Tree),
             (1, 3, graph::DFSEdgeType::Tree),
             (1, 4, graph::DFSEdgeType::Cross),
             (2, 3, graph::DFSEdgeType::Forward)];

    assert_eq!(expected_edges, edges);

}

#[test]
fn edge_processing_undirected() {
    let g = graph_builders::from_file("test_data/graph3").unwrap();

    let mut edges = Vec::<(usize, usize, graph::DFSEdgeType)>::new();

    g.depth_first_iter_from_index(|_| { },
                                  |_| { },
                                  |s, d, t| edges.push((*s, *d, t)),
                                  1);
    let expected_edges =
        vec![(1, 0, graph::DFSEdgeType::Tree),
             (0, 3, graph::DFSEdgeType::Tree),
             (3, 2, graph::DFSEdgeType::Tree),
             (2, 1, graph::DFSEdgeType::Back),
             (2, 4, graph::DFSEdgeType::Tree),
             (4, 3, graph::DFSEdgeType::Back),
             (0, 5, graph::DFSEdgeType::Tree)];

    assert_eq!(expected_edges, edges);

}

#[test]
fn iter_on_fan_in() {
    let mut g = graph_builders::unconnected(vec![0, 1, 2, 3, 4], true);
    g.add_directed_edge(0, 1);
    g.add_directed_edge(1, 2);
    g.add_directed_edge(3, 1);
    g.add_directed_edge(4, 2);

    let mut discovery_order = Vec::<usize>::new();
    let mut processed_order = Vec::<usize>::new();
    let mut edges = Vec::<(usize, usize, graph::DFSEdgeType)>::new();
    
    g.depth_first_iter(|i| discovery_order.push(*i),
                       |i| processed_order.push(*i),
                       |s, d, t| edges.push((*s, *d, t)));

    assert_eq!(vec![0, 1, 2, 3, 4], discovery_order);
    assert_eq!(vec![2, 1, 0, 3, 4], processed_order);

    let expected_edges =
        vec![(0, 1, graph::DFSEdgeType::Tree),
             (1, 2, graph::DFSEdgeType::Tree),
             (3, 1, graph::DFSEdgeType::Forward),
             (4, 2, graph::DFSEdgeType::Forward)];  // FIXME - should these really be Cross edges?

    assert_eq!(expected_edges, edges);
}
