use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

pub mod graph_builders;

pub struct Graph<T> where T : Clone + Eq + Hash {
    nodes          : Vec<T>,
    node_indices   : HashMap<T, usize>,
    node_degrees   : Vec<usize>,
    adjacency_list : Vec<Vec<usize>>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum BFSTraversalState {
    Undiscovered,
    Discovered,
    Processed,
}

#[derive(PartialEq, Eq, Clone)]
pub enum DFSTraversalState {
    Undiscovered,
    Processing (usize),
    Processed  (usize, usize),
}

#[derive(PartialEq, Eq, Clone)]
pub enum DFSEdgeType {
    Tree,
    Back,
    Cross,
    Forward,
}


impl<T> Graph<T> where T : Clone + Eq + Hash {

    pub fn new() -> Graph<T> {
        Graph { nodes : Vec::new(), node_indices : HashMap::new(), node_degrees : Vec::new(), adjacency_list : Vec::new() }
    }

    pub fn number_of_vertices(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_from_index(&self, index : usize) -> T {
        if index < self.number_of_vertices() {
            self.nodes[index].clone()
        } else {
            panic!("Called node_from_index({}) on a graph with only {} vertices!", index, self.number_of_vertices())
        }
    }

    pub fn index_from_node(&self, node : T) -> usize {
        match self.node_indices.get(&node) {
            Some(i) => i.clone(),
            None => panic!("Node was not present in the graph."),
        }
    }

    pub fn get_degree_from_index(&self, index : usize) -> usize {
        self.node_degrees[index]
    }

    pub fn add_directed_edge(&mut self, source_index : usize, dest_index : usize) {
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

    pub fn add_undirected_edge(&mut self, source_index : usize, dest_index : usize) {
        self.add_directed_edge(source_index, dest_index);
        self.add_directed_edge(dest_index, source_index);
    }

    pub fn breadth_first_iter_from_index<F, G> (&self,
                                                mut process_vertex : F,
                                                mut process_edge : G,
                                                root_index : usize)
        where F : FnMut(&T) -> (), G : FnMut(&T, &T) -> () {

        let mut node_states = vec![BFSTraversalState::Undiscovered; self.number_of_vertices()];
        node_states[root_index] = BFSTraversalState::Discovered;

        let mut nodes_to_process = VecDeque::<usize>::new();
        nodes_to_process.push_back(root_index);

        while let Some(current_node) = nodes_to_process.pop_front() {
            for dest_node in &self.adjacency_list[current_node] {
                // Note - this does both edges in both directions for an undirected graph.
                process_edge(&self.nodes[current_node], &self.nodes[*dest_node]);
                if node_states[*dest_node] == BFSTraversalState::Undiscovered {
                    node_states[*dest_node] = BFSTraversalState::Discovered;
                    nodes_to_process.push_back(*dest_node);
                }
            }

            process_vertex(&self.nodes[current_node]);
            node_states[current_node] = BFSTraversalState::Processed;
        }
    }

    // FIXME - I don't understand why process_edge and process_vertex don't need
    //         to be mutable here, whereas they do in the fn above (which I call!)
    pub fn breadth_first_iter_from_node<F, G> (&self,
                                               process_vertex : F,
                                               process_edge : G,
                                               root_node : T)
        where F : FnMut(&T) -> (), G : FnMut(&T, &T) -> () {
        let root_index = self.index_from_node(root_node);
        self.breadth_first_iter_from_index(process_vertex, process_edge, root_index)
    }

    pub fn depth_first_iter_from_index<F, G, H> (&self,
                                                 mut process_vertex_early : F,
                                                 mut process_vertex_late  : G,
                                                 mut process_edge         : H,
                                                 root_node : usize)
        where F : FnMut(&T) -> (), G : FnMut(&T) -> (), H : FnMut(&T, &T, DFSEdgeType) -> () {
        
        let mut discovery_state = vec![DFSTraversalState::Undiscovered; self.number_of_vertices()];
        // Call into a recursive function
        self.inner_dfs(&mut process_vertex_early,
                       &mut process_vertex_late,
                       &mut process_edge,
                       &mut discovery_state,
                       0usize,
                       root_node);
    }

    // Recursive part of DFS
    fn inner_dfs<F, G, H>(&self,
                          process_vertex_early     : &mut F,
                          process_vertex_late      : &mut G,
                          process_edge             : &mut H,
                          discovery_state          : &mut Vec<DFSTraversalState>,
                          time                     : usize,
                          current_node             : usize) -> usize    // Returns the exit time + 1
        where F : FnMut(&T) -> (), G : FnMut(&T) -> (),
              H : FnMut(&T, &T, DFSEdgeType) -> () {
            process_vertex_early(&self.nodes[current_node]);
            let entry_time : usize = time;
            let mut running_time = time + 1;
            discovery_state[current_node] = DFSTraversalState::Processing(entry_time);
            running_time += 1;
            for dest_node in &self.adjacency_list[current_node] {
                match discovery_state[*dest_node] {
                    DFSTraversalState::Undiscovered => {
                        //parent[*dest_node] = Some(current_node);
                        process_edge(&self.nodes[current_node], &self.nodes[*dest_node], DFSEdgeType::Tree);
                        running_time = self.inner_dfs(process_vertex_early,
                                                      process_vertex_late,
                                                      process_edge,
                                                      discovery_state,
                                                      running_time,
                                                      *dest_node);
                    },
                    DFSTraversalState::Processing(_) => {
                        process_edge(&self.nodes[current_node], &self.nodes[*dest_node], DFSEdgeType::Back);
                    },
                    DFSTraversalState::Processed(dest_entry_time, _) => {
                        if dest_entry_time > entry_time {
                            process_edge(&self.nodes[current_node], &self.nodes[*dest_node], DFSEdgeType::Forward);
                        } else {
                            process_edge(&self.nodes[current_node], &self.nodes[*dest_node], DFSEdgeType::Cross);
                        }
                    },
                }
            }
            discovery_state[current_node] = DFSTraversalState::Processed(entry_time, running_time);
            running_time += 1;
            process_vertex_late(&self.nodes[current_node]);
            // Return the next time available for use
            running_time
        }
}

