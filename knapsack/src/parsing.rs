use crate::structs::{Item, Knapsack};

// Input file:
// n K
// v_0 w_0
// v_1 w_1
pub fn parse_input_data(args: Vec<String>) -> Result<(Knapsack, Vec<Item>), String> {
    if args.is_empty() {
        panic!("knapsack/main.rs appelé sans argument")
    }

    let args_one = args[1].clone();
    let mut args_splitted = args_one.split('\n');
    if let Some(first_line) = args_splitted.next() {
        let k: Knapsack = Knapsack {
            capacity: first_line
                .split(' ')
                .nth(1)
                .unwrap_or("-1")
                .to_string()
                .parse::<i32>()
                .unwrap_or(-2),
        };

        let vi = args_splitted
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(i, s)| -> Item {
                let mut line = s.split(' ');
                Item {
                    index: i as i32,
                    v: line
                        .next()
                        .unwrap_or("-3")
                        .to_string()
                        .parse::<i32>()
                        .unwrap_or(-4),
                    w: line
                        .next()
                        .unwrap_or("-3")
                        .to_string()
                        .parse::<i32>()
                        .unwrap_or(-4),
                    is_picked: false,
                }
            })
            .collect::<Vec<Item>>();

        return Ok((k, vi));
    }

    Err(format!("Input file has bad format {:#?}", args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    macro_rules! data_path {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/data/", $fname)
        };
    }

    #[test]
    fn parse_good_file() {
        let to_test: Vec<String> = vec![
            "bin exec".to_string(),
            "4 50\n1 20\n2 30\n3 30\n4 30".to_string(),
        ];

        assert_eq!(
            parse_input_data(to_test).expect("Erreur dans parse_input_data"),
            (
                Knapsack { capacity: 50 },
                vec![
                    Item {
                        index: 0,
                        v: 1,
                        w: 20,
                        is_picked: false
                    },
                    Item {
                        index: 1,
                        v: 2,
                        w: 30,
                        is_picked: false
                    },
                    Item {
                        index: 2,
                        v: 3,
                        w: 30,
                        is_picked: false
                    },
                    Item {
                        index: 3,
                        v: 4,
                        w: 30,
                        is_picked: false
                    },
                ]
            )
        );
    }

    #[test]
    fn parse_files() {
        let file_path = data_path!("ks_10000_0");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let first_asn = contents.find('\n').unwrap();

        let mut first_line = contents.clone();
        first_line.truncate(first_asn);

        let first_line_vec = first_line.split_whitespace().collect::<Vec<&str>>();
        let expected_n = first_line_vec.get(0).unwrap();
        let expected_k = first_line_vec.get(1).unwrap();

        let (actual_k, actual_vi) =
            parse_input_data(vec!["".to_string(), contents, "".to_string()])
                .expect("Problème dans parse_input_data");

        // récupérer la taille de k pour comparer
        assert_eq!(actual_k.capacity, expected_k.parse::<i32>().unwrap());

        // récupérer le nombre de n pour comparer la taille de vi
        assert_eq!(actual_vi.len(), expected_n.parse::<usize>().unwrap());

        let vi_first = actual_vi.get(0).unwrap();
        let vi_last = actual_vi.get(actual_vi.len() - 1).unwrap();
        let vi_sixty = actual_vi.get(59).unwrap();

        assert_eq!(vi_first.index, 0);
        assert_eq!(vi_first.v, 122416);
        assert_eq!(vi_first.w, 120553);

        assert_eq!(vi_sixty.index, 59);
        assert_eq!(vi_sixty.v, 154292);
        assert_eq!(vi_sixty.w, 158464);

        assert_eq!(vi_last.index, (actual_vi.len() - 1) as i32);
        assert_eq!(vi_last.v, 152937);
        assert_eq!(vi_last.w, 145145);

        actual_vi.into_iter().for_each(|item: Item| {
            assert!(item.index >= 0);
            assert!(item.v > 0);
            assert!(item.w > 0);
        });
    }
}
