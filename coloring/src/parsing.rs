
fn extract_usize(s: String) -> Vec<usize> {
    s.split_whitespace().map(|f| {
        f.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>()
}

pub fn parse_input_data(args: Vec<String>) {
    let args_one = args[1].clone();
    let mut args_splitted = args_one.split('\n');

    if let Some(first_line) = args_splitted.next() {
        let first_line_integers = extract_usize(first_line.to_string());

        if first_line_integers.len() != 2 {
            panic!("Erreur, devrait avoir deux integer par ligne!");
        }

        let number_of_nodes = first_line_integers[0];
        let number_of_edges = first_line_integers[1];

        // TODO : here, en train de parse les nodes.
        // Appris : extract_usize() !
    }
}