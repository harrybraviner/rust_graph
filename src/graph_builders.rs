use super::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

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

pub fn from_file(filename : &str) -> Result<Graph<usize>> {
//    match File::open(filename) {
//        Ok(f) => {
//            let mut buf_reader = BufReader::new(f);
//            let mut contents = String::new();
//            // Read the header
//            buf_reader.read_to_string(&mut contents);
//            // FIXME - want to match on the above, but really ugly since I'll have lots of matches.
//            //         What's the idiomatic way to actually do this?
//            panic!("Not implemented")
//        },
//        Err(_) => Err(String::from("Unable to open file.")),
//    }
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    // Read the header line
    buf_reader.read_line(&mut contents)?;

    // Read the number of vertices
    contents.clear();
    buf_reader.read_line(&mut contents)?;
    if &contents[0..20] != "number_of_vertices: " {
        panic!("Second line of graph file is {}, which is not of correct format: number_of_vertices: n", &contents)
    }
    let number_of_vertices : usize = str::parse(&contents.trim()[20..]).unwrap();

    // Read whether or not the graph is directed
    contents.clear();
    buf_reader.read_line(&mut contents)?;
    if &contents[0..10] != "directed: " {
        panic!("Third line of graph file is {}, which is not of correct format: directed: [true/false]", &contents)
    }
    let directed : bool = str::parse(&contents.trim()[10..]).unwrap();

    // Read edges
    let mut edges : Vec<(usize, usize)> = Vec::new();
    loop {
        contents.clear();
        buf_reader.read_line(&mut contents)?;
        let mut iter = contents.split_whitespace();
        let source = iter.next();
        let dest = iter.next();
        match (source, dest) {
            (Some(source), Some(dest)) => {
                edges.push((str::parse(&source).unwrap(), str::parse(&dest).unwrap()))
            },
            _ => break,
        }
    }

    let mut g = unconnected((0..number_of_vertices).collect());
    for (s, d) in edges {
        if directed {
            g.add_directed_edge(s, d);
        } else {
            g.add_undirected_edge(s, d);
        }
    }

    return Ok(g);
}
