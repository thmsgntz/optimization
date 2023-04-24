use std::env;
mod parsing;
mod structs;

use crate::parsing::parse_input_data;
use crate::structs::{Item, Items, Knapsack, DynamicProg, print_result};

fn dynamic_programing(sack: Knapsack, vitems: &mut Vec<Item>) {
    let capacity = sack.capacity as usize;

    let mut dynamic_prog = DynamicProg::new(capacity, vitems.len());
    dynamic_prog.fill(capacity, vitems);
    let solution = dynamic_prog.solve(vitems);

    println!("{}", print_result(vitems, true, solution));
}

fn greedy(sack: Knapsack, vitems: Vec<Item>) {
    // 1. sort items by decreasing weight
    // 2. take as much as possible
    let mut items = Items(vitems);
    items.sort_by_weight();
    let mut k = sack.capacity;

    // println!("Size: {}", k);
    // println!("Sorted: {:?}", items);

    // si on prend un item
    // on met son item.index à 1 dans une chaine de byte à 0
    // pour savoir ceux qu'on prend, on regarde les indices des 1
    let picked_values: i32 = items
        .0
        .iter_mut()
        .map(|mut item| -> i32 {
            if k - item.w > 0 {
                k -= item.w;
                item.is_picked = true;
                return item.v;
            }
            0
        })
        .sum();

    println!("{} 0", picked_values);
    items.sort_by_index();
    for item in items.0.iter() {
        print!("{} ", item.is_picked as usize);
    }
    println!();
}

fn main() {
    let k: Result<(Knapsack, Vec<Item>), String> = parse_input_data(env::args().collect());

    match k {
        Err(v) => eprintln!("{}", v),
        Ok((kn, mut vi)) => {
            //println!("Knap: {:#?}", kn);
            //println!("V<item>: {:#?}", vi);

            if vi.len() > 5000 {
                greedy(kn, vi);
            }
            else {
                dynamic_programing(kn, &mut vi);
            }
        }
    }
}