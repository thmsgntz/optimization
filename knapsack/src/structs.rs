use std::fmt;

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

#[cfg(test)]
mod tests {}
