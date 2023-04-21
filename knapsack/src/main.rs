use std::env;
mod parsing;
mod structs;

use crate::parsing::parse_input_data;
use crate::structs::{Item, Items, Knapsack};

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
        .take_while(|item| {
            if k - item.w > 0 {
                k -= item.w;
                return true;
            }

            false
        })
        .map(|mut item| -> i32 {
            item.is_picked = true;
            item.v
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
        Ok((kn, vi)) => {
            //println!("Knap: {:#?}", kn);
            //println!("V<item>: {:#?}", vi);

            greedy(kn, vi);
        }
    }
}
