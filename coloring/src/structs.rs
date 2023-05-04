#[derive(Debug, PartialEq)]
pub struct Color(i32);

impl Color {
    pub fn new() -> Self {
        Self(-1)
    }
}

impl Into<Color> for usize {
    fn into(self) -> Color {
        Color(self as i32)
    }
}

pub struct Constraint {
    domains: Vec<Vec<Color>>,
    decision_variables: Vec<Color>,
}

// TODO HERE: on crée le Constraint Global. Domains+Decisions variables
//    - ajouter: check_feasibility / prune
impl Constraint {
    pub fn new(number_of_nodes: usize, higest_ranking: usize) -> Self {
        Self {
            domains: (0..number_of_nodes)
                .map(|_| -> Vec<Color> {
                    (0..higest_ranking)
                        .map(|i| -> Color { i.into() })
                        .collect::<Vec<Color>>()
                })
                .collect::<Vec<Vec<Color>>>(),

            decision_variables: (0..number_of_nodes)
                .map(|_| -> Color { Color::new() })
                .collect::<Vec<Color>>(),
        }
    }
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(number_of_nodes: usize, number_of_edges: usize) -> Self {
        Self {
            nodes: (0..number_of_nodes)
                .map(|i: usize| -> Node { Node::new(i) })
                .collect::<Vec<Node>>(),
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, n1: usize, n2: usize) {
        self.nodes[n1].link_to.push(n2);
        self.nodes[n2].link_to.push(n1);

        self.edges.push(vec![n1, n2]);
    }

    pub fn get_node_i(&self, index: usize) -> &Node {
        &self.nodes[index]
    }
}

#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub selected_color: Color,
    pub link_to: Vec<usize>,
}

impl Node {
    fn new(i: usize) -> Self {
        Self {
            id: i,
            selected_color: Color::new(),
            link_to: Vec::new(),
        }
    }

    fn add(&mut self, node_index: usize) {
        self.link_to.push(node_index);
    }

    pub fn get_rank(&self) -> usize {
        self.link_to.len()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[cfg(test)]
mod tests {
    use super::{Constraint, Graph, Node};

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
        let expected_n: usize = first_line_vec
            .get(0)
            .expect("Expected |N| here")
            .parse::<usize>()
            .unwrap();
        let expected_e = first_line_vec
            .get(1)
            .expect("Expected |E| here")
            .parse::<usize>()
            .unwrap();

        let mut graph = Graph::new(expected_n, expected_e);

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

    #[test]
    fn contrainst_new() {
        let nb_nodes = 5;
        let highest_rank = 3;

        let actual = Constraint::new(nb_nodes, highest_rank);
        let actual_domain = actual.domains;
        let actual_dv = actual.decision_variables;

        assert_eq!(actual_domain.len(), nb_nodes);

        let expected_domain_vector = vec![0.into(), 1.into(), 2.into()];
        assert_eq!(
            actual_domain
                .into_iter()
                .take_while(|v| -> bool { v == &expected_domain_vector })
                .count(),
            nb_nodes,
            "Le domaine initial pour chaque noeud n'est pas bon. (devrait être 0..highest_rank)"
        );

        assert_eq!(actual_dv.len(), nb_nodes);
        assert_eq!(
            actual_dv
                .into_iter()
                .take_while(|item| -> bool { item.0 == -1 })
                .count(),
            nb_nodes,
            "chaque decision variable est initialisée à Color(-1) (color::new)"
        );
    }
}
