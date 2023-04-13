use std::env;
mod parsing;
mod structs;

use crate::parsing::parse_input_data;
use crate::structs::{Item, Knapsack};

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
