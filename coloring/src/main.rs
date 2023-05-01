mod parsing;

use std::env;

#[derive(Debug)]
enum Color {
    DEFAULT,
    RED,
    BLUE,
    GREEN
}

struct Graph(Vec<Node>);

impl Graph {
    fn new (number_of_nodes: usize) -> Self {
        Self((0..number_of_nodes).map(|i: usize| -> Node {
            Node::new(i)
        } ).collect())
    }

    fn add_edge(&mut self, n1: usize, n2: usize) {
        self.0[n1].link_to.push(n2);
        self.0[n2].link_to.push(n1);
    }

    fn get_node_i(&self, index: usize) -> &Node {
        &self.0[index]
    }
}

#[derive(Debug)]
struct Node {
    id: usize,
    selected_color: Color,
    link_to: Vec<usize>,
}

impl  Node {
    fn new(i: usize) -> Self {
        Self { id: i, selected_color: Color::DEFAULT, link_to: Vec::new() }
    }

    fn add(&mut self, node_index: usize) {
        self.link_to.push(node_index);
    }
}

impl  PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}



#[cfg(test)]
mod tests {
    use super::{Node, Color, Graph};

    use std::fs;

    macro_rules! data_path {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/data/", $fname)
        };
    }

    #[test]
    fn node_new() {
        let mut n1 = Node::new(0);
        let n2 = Node::new(1);

        n1.add(n2.id);

        assert!(n1.link_to.len() > 0);
        assert_eq!(n1.link_to[0], n2.id);
    }

    #[test]
    fn node_creation_file() {
        let file_path = data_path!("gc_4_1");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let first_asn = contents.find('\n').unwrap();
        let mut first_line = contents.clone();
        first_line.truncate(first_asn);

        let first_line_vec = first_line.split_whitespace().collect::<Vec<&str>>();
        let expected_n: usize = first_line_vec.get(0).expect("Expected |N| here").parse::<usize>().unwrap();
        let expected_e = first_line_vec.get(1).expect("Expected |E| here").parse::<usize>().unwrap();

        let mut graph = Graph::new(expected_n);

        // eprintln!("#Nodes: {}, #Edges: {}", expected_n, expected_e);
        let mut lines = contents.split('\n').skip(1);
        for _ in 0..expected_e {
            let line = lines.next().unwrap();

            // eprint!("i'{}'\n",line);

            let v = line.split_whitespace().collect::<Vec<&str>>();
            let id_n1 = v.get(0).unwrap().parse::<usize>().unwrap();
            let id_n2 = v.get(1).unwrap().parse::<usize>().unwrap();

            graph.add_edge(id_n1, id_n2);
        }

        // eprintln!("Node 0 {:#?}", graph.get_node_i(0));
        assert_eq!(graph.get_node_i(0).link_to, [1]);
        assert_eq!(graph.get_node_i(1).link_to, [0, 2, 3]);
        assert_eq!(graph.get_node_i(2).link_to, [1]);
        assert_eq!(graph.get_node_i(3).link_to, [1]);

    }
}


//  |N| |E|
//  u_0 v_0
//  u_1 v_1
//  ...
//  u_|E|-1 v_|E|-1
fn parse_input_data(args: Vec<String>) {
    if args.is_empty() {
        panic!("coloring/main.rs appelé sans argument")
    }

    let args_one = args[1].clone();
    let mut args_splitted = args_one.split('\n');

}

fn main() {
    parse_input_data(env::args().collect());

    println!("Hello, world!");
}
