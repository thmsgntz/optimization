use std::fs;

macro_rules! data_path {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/data_tests/", $fname)
    };
}

// Return file content into one string "4 5\n5 8\n"
pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn split_and_parse_usize(file_as_string: String, sep: char) -> Vec<Vec<usize>> {
    file_as_string
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| -> Vec<usize> {
            line.split(sep)
                .map(|value| -> usize {
                    value
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Could not parse {value}"))
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
mod tests {
    use super::{read_file, split_and_parse_usize};

    #[test]
    fn read_file_call() {
        let file_path = data_path!("ks_4_0");
        let actual = read_file(file_path);
        assert_eq!(actual, "4 11\n8 4\n10 5\n15 8\n4 3\n");
    }

    #[test]
    fn split_and_parse_call() {
        let file_path = data_path!("ks_4_0");
        let file_as_string = read_file(file_path);

        let actual = split_and_parse_usize(file_as_string, ' ');

        assert_eq!(
            actual,
            vec![
                vec![4, 11],
                vec![8, 4],
                vec![10, 5],
                vec![15, 8],
                vec![4, 3],
            ]
        )
    }
}
