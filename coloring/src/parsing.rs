use crate::structs::Graph;
use parsing::split_and_parse_usize;

pub fn parse_input_data(args: Vec<String>) -> Graph {
    let data_file = split_and_parse_usize(args[1].clone(), ' ');

    let first_line = &data_file[0];
    let number_of_nodes = first_line[0];
    let number_of_edges = first_line[1];

    let mut graph = Graph::new(number_of_nodes, number_of_edges);

    let edges_read = data_file[1..]
        .into_iter()
        .map(|vec| {
            graph.add_edge(vec[0], vec[1]);
            1
        })
        .sum::<usize>();

    assert_eq!(number_of_edges, edges_read);

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{Color, Node};
    use parsing::read_file;

    #[test]
    fn parse_input_data_call() {
        let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/gc_4_1");

        let actual = parse_input_data(vec!["".to_string(), read_file(file_path), "".to_string()]);
        let expected_n1 = Node {
            id: 0,
            selected_color: Color::new(),
            link_to: vec![1],
        };

        let expected_n2 = Node {
            id: 1,
            selected_color: Color::new(),
            link_to: vec![0, 2, 3],
        };

        let expected_n3 = Node {
            id: 2,
            selected_color: Color::new(),
            link_to: vec![1],
        };

        let expected_n4 = Node {
            id: 3,
            selected_color: Color::new(),
            link_to: vec![1],
        };

        // test nodes
        assert_eq!(actual.nodes.len(), 4);

        for (i, expected_node) in [expected_n1, expected_n2, expected_n3, expected_n4]
            .iter()
            .enumerate()
        {
            let n = &actual.nodes[i];
            assert_eq!(n.id, expected_node.id);
            assert_eq!(n.selected_color, expected_node.selected_color);
            assert_eq!(n.link_to, expected_node.link_to);
        }

        // test edges
        assert_eq!(actual.edges.len(), 3);
        assert_eq!(actual.edges, vec![vec![0, 1], vec![1, 2], vec![1, 3]]);
    }

    #[test]
    fn parse_input_gc_20_1() {
        let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/gc_20_1");

        let actual = parse_input_data(vec!["".to_string(), read_file(file_path), "".to_string()]);

        // nodes
        let actual_nodes = &actual.nodes;
        assert_eq!(actual_nodes.len(), 20);

        assert_eq!(actual_nodes[0].id, 0);
        assert_eq!(actual_nodes[0].link_to, [16]);

        assert_eq!(actual_nodes[1].id, 1);
        assert_eq!(actual_nodes[1].link_to, [2, 6, 7, 8]);

        assert_eq!(actual_nodes[16].id, 16);
        assert_eq!(actual_nodes[16].link_to, [0, 2, 3, 19]);

        assert_eq!(actual_nodes[19].id, 19);
        assert_eq!(actual_nodes[19].link_to, [16]);

        // edges
        let actual_edges = &actual.edges;
        assert_eq!(actual_edges.len(), 23);
        assert_eq!(
            actual_edges,
            &vec![
                vec![0, 16],
                vec![1, 2],
                vec![1, 6],
                vec![1, 7],
                vec![1, 8],
                vec![2, 11],
                vec![2, 16],
                vec![2, 17],
                vec![3, 14],
                vec![3, 16],
                vec![3, 17],
                vec![4, 7],
                vec![4, 13],
                vec![4, 17],
                vec![5, 6],
                vec![5, 11],
                vec![6, 18],
                vec![9, 12],
                vec![10, 13],
                vec![11, 17],
                vec![13, 15],
                vec![15, 17],
                vec![16, 19],
            ]
        );
    }
}
