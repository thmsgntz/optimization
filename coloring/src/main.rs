mod parsing;
mod structs;

use std::env;

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
