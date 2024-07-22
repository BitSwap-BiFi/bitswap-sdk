use crate::Pair;

impl Pair {
    pub fn new(a: u8, b: u8) -> Self {
        Pair { a, b }
    }
}
pub fn pairs(xs: &[u8]) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            pairs.push(Pair::new(xs[i], xs[j]));
        }
    }
    pairs
}

impl rgb2020::Pair for Pair {
    fn get_a(&self) -> u8 {
        self.a
    }
    fn get_b(&self) -> u8 {
        self.b
    }
}
