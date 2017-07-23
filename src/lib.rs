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

    pub fn depth_first_iter_from_index<F, G> (&self,
                                              process_vertex_early : F,
                                              process_vertex_late  : F,
                                              process_edge         : G,
                                              root_node : usize)
        where F : FnMut(&T) -> (), G : FnMut(&T, &T) -> () {

        let mut discovery_state = vec![DFSTraversalState::Undiscovered; self.number_of_vertices()];
        let mut parent : Vec<Option<usize>> = vec![None; self.number_of_vertices()];
        let mut time = 0usize;  // FIXME - if we have more than MAX_USIZE/2 nodes in the graph, this will overflow!
        let mut processing_stack = VecDeque::<usize>::new();

        // Setup out stack so that we start with the specified root node
        //processing_stack.push_back(root_node);

        // Keeping an explict stack turns out to be quite tricky.
        // A recursive function (that changes all of the above mutable state)
        // seems like a much nicer way of doing this.
        // FIXME - calls to processing functions?
        // FIXME - make the recursion below work!
        fn inner_dfs(current_node    : usize,
                     adjacency_list  : &    Vec<Vec<usize>>,
                     discovery_state : &mut Vec<DFSTraversalState>,
                     parent          : &mut Vec<Option<usize>>,
                     time            : &mut usize) {
            let entry_time : usize = *time;
            discovery_state[current_node] = DFSTraversalState::Processing(entry_time);
            *time += 1;
            for dest_node in &adjacency_list[current_node] {
                if discovery_state[*dest_node] == DFSTraversalState::Undiscovered {
                    parent[*dest_node] = Some(current_node);
                    inner_dfs(*dest_node, adjacency_list, discovery_state, parent, time);
                    // FIXME - process_edge??
                }
            }
            discovery_state[current_node] = DFSTraversalState::Processed(entry_time, *time);
            *time += 1;
        };

        inner_dfs(root_node, &self.adjacency_list, &mut discovery_state, &mut parent, &mut time);

//        while let Some(current_node) = processing_stack.pop_back() {
//            discovery_state[current_node] = DFSTraversalState::Processing(time);
//            time += 1;
//            for dest_node in &self.adjacency_list[current_node] {
//                if discovery_state[*dest_node] == DFSTraversalState::Undiscovered {
//                    parent[*dest_node] = Some(current_node);    // FIXME - this doesn't work because of processing order!
//                    processing_stack.push_back(*dest_node);     // FIXME - and this will put some nodes of the stack multiple times!
//                }
//            }
//        }

        panic!("Not implemented")
    }
}

