#[derive(Debug, PartialEq, Eq)]
pub struct Knapsack {
    pub capacity: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub v: i32,
    pub w: i32,
}
