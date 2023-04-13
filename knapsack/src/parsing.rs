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

        let vi: Vec<Item> = args_splitted
            .filter(|s| !s.is_empty())
            .map(|s: &str| -> Item {
                let mut line = s.split(' ');
                Item {
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
                    Item { v: 1, w: 20 },
                    Item { v: 2, w: 30 },
                    Item { v: 3, w: 30 },
                    Item { v: 4, w: 30 },
                ]
            )
        );
    }

    #[test]
    fn parse_files() {
        let file_path = data_path!("ks_60_0");
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

        assert_eq!(vi_first.v, 90000);
        assert_eq!(vi_first.w, 90001);

        assert_eq!(vi_last.v, 82500);
        assert_eq!(vi_last.w, 82501);

        actual_vi.into_iter().for_each(|item: Item| {
            assert!(item.v > 0);
            assert!(item.w > 0);
        });
    }
}
