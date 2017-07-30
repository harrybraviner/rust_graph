extern crate regex;

use super::Graph;
use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use self::regex::Regex;

pub fn unconnected<T : Eq + Clone + Hash>(nodes : Vec<T>, directed : bool) -> Graph<T> {
    let hash_map : HashMap<T, usize> =
        nodes.iter()
             .cloned()
             .enumerate()
             .map(|(i, x)| { (x, i) })
             .collect();
    let adjacency_list = vec![Vec::new(); nodes.len()];
    Graph { nodes : nodes.clone(), directed : directed, node_indices : hash_map, adjacency_list : adjacency_list }
}

pub fn from_file(filename : &str) -> Result<Graph<usize>> {
    let (number_of_vertices, directed, edges, _) = parse_file::<usize>(filename);

    let mut g = unconnected((0..number_of_vertices).collect(), directed);

    for (source, dest) in edges {
        if directed { g.add_directed_edge(source, dest) }
        else { g.add_undirected_edge(source, dest) }
    }

    Ok(g)
}

// A helper function that returns the parsed data read from the graph file.
// We want to do slightly different things with it depending on whether or not
// we're expecting node names.
fn parse_file<T>(filename : &str) -> (usize, bool, Vec<(usize, usize)>, Vec<T>)  {
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    // Strip out comments and blank lines
    let lines =
        buf_reader.lines()
                  .map(|l| l.unwrap())  // FIXME - maybe better error handling than this?
                  .map(|l| l.split("//").next().map(|x| String::from(x.trim())))
                  .filter(|l| match l {
                      &None => false,
                      &Some(ref l) => l.len() > 0,
                  })
                  .map(|x| x.unwrap());

    let variable_regex = Regex::new(r"^\s*([a-z_]+)\s*:\s*([a-zA-Z0-9]*)\s*$").unwrap();
    let edges_regex = Regex::new(r"^\s*(\d+)\s*(\d+)\s*$").unwrap();
    let nodes_regex = Regex::new(r"^\s*([a-zA-Z0-9_(::)]*[a-zA-Z0-9_]+)\s*$").unwrap();

    let mut number_of_vertices = None;
    let mut directed = None;
    let mut edges_mode = false;
    let mut nodes_mode = false;
    let mut parsed_line = false;

    let mut edges = Vec::new();
    let mut nodes : Vec<T> = Vec::new();

    for line in lines {
        parsed_line = false;

        variable_regex.captures(&line).map(|cap| {
            match &cap[1] {
                "edges" => {
                    if &cap[2] != "" { panic!("Spurious text {} after edges keyword", &cap[1]) }
                    edges_mode = true;
                    parsed_line = true;
                },
                "nodes" => {
                    if &cap[2] != "" { panic!("Spurious text {} after nodes keyword", &cap[1]) }
                    nodes_mode = true;
                    parsed_line = true;
                },
                "number_of_vertices" => {
                    if number_of_vertices.is_some() { panic!("File specifies number_of_vertices multiple times.") }
                    number_of_vertices = Some((&cap[2]).parse::<usize>().unwrap());
                    parsed_line = true;
                },
                "directed" => {
                    if directed.is_some() { panic!("File specifies directed multiple times.") }
                    directed = Some((&cap[2]).parse::<bool>().unwrap());
                    parsed_line = true;
                },
                s => panic!("Unrecognised variable name: {}", &cap[1]),
            }
        });

        nodes_regex.captures(&line).map(|cap| {
            if nodes_mode {
                //let node = (&cap[
            }
        });

        edges_regex.captures(&line).map(|cap| {
            if edges_mode { // panic!("Found line formatted as edges before 'edges:' line: {}", line) }
                let source = (&cap[1]).parse::<usize>().unwrap();
                let dest   = (&cap[2]).parse::<usize>().unwrap();
                edges.push((source, dest));
                parsed_line = true;
            }
        });

        if !parsed_line { panic!("Failed to parse line: {}", line) };
    }

    let number_of_vertices = match number_of_vertices {
        None => panic!("number_of_vertices not specified"),
        Some(n) => n,
    };
    let directed = match directed {
        None => true,
        Some(d) => d,
    };

    (number_of_vertices, directed, edges, nodes)
}

pub fn make_serialization_string<T>(graph : &Graph<T>) -> String
    where T : Clone + Eq + Hash    
{
    let mut ser = String::new();
    ser.push_str("// Graph\n");
    ser.push_str(&*format!("number_of_vertices: {}\n", graph.number_of_vertices()));
    ser.push_str(&*format!("directed: {}\n", graph.is_directed()));
    
    for source in 0..(graph.number_of_vertices()) {
        for dest in &graph.adjacency_list[source] {
            if graph.is_directed() || *dest > source {
                ser.push_str(&*format!("{} {}\n", source, *dest));
            }
        }
    }

    ser
}
