extern crate graph;
use graph::graph_builders;

#[test]
fn traversal_order() {
    let g = graph_builders::from_file("test_data/graph1").unwrap();

    let mut discovery_order = Vec::<usize>::new();
    let mut processed_order = Vec::<usize>::new();

    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _| { },
                                  0);
    assert_eq!(vec![0, 1], discovery_order);
    assert_eq!(vec![1, 0], processed_order);

    discovery_order.clear();
    processed_order.clear();
    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _| { },
                                  1);
    assert_eq!(vec![1, 0], discovery_order);
    assert_eq!(vec![0, 1], processed_order);

    discovery_order.clear();
    processed_order.clear();
    g.depth_first_iter_from_index(|i| discovery_order.push(*i),
                                  |i| processed_order.push(*i),
                                  |_, _| { },
                                  2);
    assert_eq!(vec![2, 0, 1], discovery_order);
    assert_eq!(vec![1, 0, 2], processed_order);
}
