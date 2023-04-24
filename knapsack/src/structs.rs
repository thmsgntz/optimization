use std::{fmt, cmp};
use ndarray::Array2;

pub struct DynamicProg {
    pub tab: Array2<i32>,
}

impl DynamicProg {
    pub fn new(capacity: usize, n_items: usize) -> Self {
        Self { tab: Array2::<i32>::zeros((capacity + 1, n_items + 1)) }
    }

    // fill the table capacity/items=value
    pub fn fill(&mut self, capacity: usize, vitems: &Vec<Item>) {
        let tab = &mut self.tab;
        let n_items = vitems.len();

        // println!("{}, {}", capacity, n_items);
        // println!("{:?}, {:?}", tab.shape(), &[capacity, n_items + 1]);

        if tab.shape() != [capacity + 1, n_items + 1] {
            panic!("Should call DynamicProg::new() first");
        }

        for i in 0..capacity+1 {
            for j in 1..n_items+1 {
                let item: &Item = vitems.get(j - 1).expect("Should work");

                if item.w <= i as i32 {
                    // l'objet actuel rentre
                    // ((i + 1) as i32) - item.w
                    let value_item_actuel: i32 = item.v;
                    let value_item_prec: i32 = tab[[i, j - 1]];

                    let value_actuel_and_prec = if (i as i32) - item.w >= 0 {
                        tab[[i - (item.w as usize), j - 1]]
                    } else {
                        0
                    };

                    tab[[i, j]] = cmp::max(value_item_prec, value_item_actuel + value_actuel_and_prec);
                } else {
                    // est-ce qu'on peut mettre un objet précédent ?
                    tab[[i, j]] = tab[[i, j - 1]];
                }
            }
        }
    }

    // traceback the highest value from the table filled
    // and set corresponding items to picked
    pub fn solve(self, vitems: &mut [Item]) -> i32 {
        let tab = &self.tab;
        let nrows: usize = tab.nrows();
        let ncols: usize = tab.ncols();

        let solution = self.tab[[nrows - 1, ncols - 1]];

        let mut current_row = nrows - 1;
        let mut current_col = ncols - 1;
        let mut current_value = solution;

        // println!("{}", tab);

        while current_row > 0 && current_col > 0 {
            // println!("i:{}, j:{}, looking at value {}", current_row, current_col, current_value);

            if current_value != tab[[current_row, current_col - 1]] {
                // item not picked
                // current_row -= 1;
            // } else {
                // item picked
                let item = vitems.get_mut(current_col - 1).unwrap_or_else(|| panic!("Expected item as position {current_col}"));
                // println!("From value {}, we pick item j={} with index={}", current_value, current_col, item.index);
                item.is_picked = true;

                current_row -= item.w as usize;
            }

            current_col -= 1;
            current_value = tab[[current_row, current_col]]
        }

        solution
    }
}

#[cfg(test)]
mod tests_dynamic_prog {
    use ndarray::arr2;
    use super::DynamicProg;
    use super::{Item, Items};
    use super::Array2;

    #[test]
    fn create_dp() {
        let capacity = 3;
        let n_items = 6;

        let actual = DynamicProg::new(capacity, n_items);

        assert_eq!(actual.tab.len(), (capacity + 1) * (n_items + 1));
        assert_eq!(actual.tab.len_of(ndarray::Axis(0)), capacity + 1);
        assert_eq!(actual.tab.len_of(ndarray::Axis(1)), (n_items + 1));

        assert_eq!(actual.tab.sum(), 0);
    }

    #[test]
    #[should_panic]
    fn fill_dyp() {
        let mut actual = DynamicProg{tab: Array2::<i32>::eye(2)};

        actual.fill(5, &vec![
            Item{index:0, v:1, w:2, is_picked:false},
            Item{index:1, v:1, w:2, is_picked:false},
        ]);
    }

    #[test]
    fn fill_example_1() {
        let vitems = vec![
            Item{index:1, v: 5, w: 4, is_picked: false},
            Item{index:2, v: 6, w: 5, is_picked: false},
            Item{index:3, v: 3, w: 2, is_picked: false},
        ];

        let a = arr2(&[[0, 0, 0, 0],  // 0
                       [0, 0, 0, 0],  // 1
                       [0, 0, 0, 3],  // 2
                       [0, 0, 0, 3],  // 3
                       [0, 5, 5, 5],  // 4
                       [0, 5, 6, 6],  // 5
                       [0, 5, 6, 8],  // 6
                       [0, 5, 6, 9],  // 7
                       [0, 5, 6, 9],  // 8
                       [0, 5, 11, 11] // 9
        ]);

        let capacity = 9;
        let mut actual = DynamicProg::new(capacity, vitems.len());
        actual.fill(capacity, &vitems);

        assert_eq!(actual.tab, a);
    }

    #[test]
    fn fill_example_2() {
        let vitems = vec![
            Item{index:1, v: 16, w: 2, is_picked: false},
            Item{index:2, v: 19, w: 3, is_picked: false},
            Item{index:3, v: 23, w: 4, is_picked: false},
            Item{index:4, v: 28, w: 5, is_picked: false},
        ];

        let b = arr2(&[[0, 0, 0, 0, 0],  // 0
                       [0, 0, 0, 0, 0],  // 1
                       [0, 16, 16, 16, 16],  // 2
                       [0, 16, 19, 19, 19],  // 3
                       [0, 16, 19, 23, 23],  // 4
                       [0, 16, 35, 35, 35],  // 5
                       [0, 16, 35, 39, 39],  // 6
                       [0, 16, 35, 42, 44],  // 7
        ]);

        let capacity = 7;
        let mut actual = DynamicProg::new(capacity, vitems.len());
        actual.fill(capacity, &vitems);

        assert_eq!(actual.tab, b);
    }

    #[test]
    fn solve_example_1() {
        let mut vitems = vec![
            Item{index:1, v: 5, w: 4, is_picked: false},
            Item{index:2, v: 6, w: 5, is_picked: false},
            Item{index:3, v: 3, w: 2, is_picked: false},
        ];

        let capacity = 9;
        let mut actual = DynamicProg::new(capacity, vitems.len());
        actual.fill(capacity, &vitems);

        let solution = actual.solve(&mut vitems);

        let actual_picked_items = vitems.iter().filter(|item| item.is_picked).collect::<Vec<&Item>>();
        assert_eq!(actual_picked_items.len(), 2);
        assert_eq!(solution, 11);

        assert!(matches!(actual_picked_items.get(0).expect("msg").index, 1 | 2));
        assert!(matches!(actual_picked_items.get(1).expect("msg").index, 1 | 2));

        assert_eq!(actual_picked_items.iter().map(|item| item.v).sum::<i32>(), solution);

        let expected_output = "11 1\n1 1 0\n";
        assert_eq!(expected_output, Items(vitems).print_result(true, solution));
    }

#[test]
    fn solve_example_2() {
        let mut vitems = vec![
            Item{index:1, v: 16, w: 2, is_picked: false},
            Item{index:2, v: 19, w: 3, is_picked: false},
            Item{index:3, v: 23, w: 4, is_picked: false},
            Item{index:4, v: 28, w: 5, is_picked: false},
        ];

        let capacity = 7;
        let expected_solution = 44;
        let mut actual = DynamicProg::new(capacity, vitems.len());
        actual.fill(capacity, &vitems);

        let solution = actual.solve(&mut vitems);

        let actual_picked_items = vitems.iter().filter(|item| item.is_picked).collect::<Vec<&Item>>();
        assert_eq!(actual_picked_items.len(), 2);
        assert_eq!(solution, expected_solution);

        assert!(matches!(actual_picked_items.get(0).expect("msg").index, 1 | 4));
        assert!(matches!(actual_picked_items.get(1).expect("msg").index, 1 | 4));

        assert_eq!(actual_picked_items.iter().map(|item| item.v).sum::<i32>(), expected_solution);

        let expected_output = "44 1\n1 0 0 1\n";
        assert_eq!(expected_output, Items(vitems).print_result(true, solution));
    }
}





#[derive(Debug, PartialEq, Eq)]
pub struct Knapsack {
    pub capacity: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub index: i32,
    pub v: i32,
    pub w: i32,
    pub is_picked: bool,
}

#[derive(Debug)]
pub struct Items(pub Vec<Item>);

impl Items {
    pub fn sort_by_weight(&mut self) {
        self.0.sort_unstable_by_key(|item| item.w);
    }

    pub fn sort_by_index(&mut self) {
        self.0.sort_unstable_by_key(|item| item.index);
    }

    pub fn sort_by_density(&mut self) {
        self.0.sort_unstable_by_key(|item| item.v/item.w);
    }

    pub fn print_result(&mut self, is_optimal: bool, solution: i32) -> String {
        let output = format!("{} {}", solution, is_optimal as usize);

        self.sort_by_index();
        let s: Vec<String> = self.0.iter().map(|item| format!("{}", item.is_picked as usize)).collect::<Vec<String>>();
        format!("{}\n{}\n", output, s.join(" "))
    }
}

pub fn print_result(vitems: &mut [Item], is_optimal: bool, solution: i32) -> String {
    let output = format!("{} {}", solution, is_optimal as usize);

    vitems.sort_unstable_by_key(|item| item.index);
    let s: Vec<String> = vitems.iter().map(|item| format!("{}", item.is_picked as usize)).collect::<Vec<String>>();
    format!("{}\n{}\n", output, s.join(" "))
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Item [#{}, v: {}, w:{}, p:{}]",
            self.index, self.v, self.w, self.is_picked
        )
    }
}

