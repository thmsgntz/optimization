use std::env;

#[derive(Debug)]
struct Knapsack {
    capacity: i32,
}

#[derive(Debug)]
struct Item {
    v: i32,
    w: i32,
}

// Input file:
// n K
// v_0 w_0
// v_1 w_1
fn parse_input_data(args: Vec<String>) -> Result<(Knapsack, Vec<Item>), String> {
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

    Err(format!("Input file as bad format {:#?}", args))
}

fn main() {
    let k: Result<(Knapsack, Vec<Item>), String> = parse_input_data(env::args().collect());

    match k {
        Err(v) => eprintln!("{}", v),
        Ok((kn, vi)) => {
            println!("Knap: {:#?}", kn);
            println!("V<item>: {:#?}", vi);
        }
    }
}
