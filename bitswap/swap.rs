use crate::bitswap_core::Bitswap;
use crate::Swap;
use bitswap::swap::{Pairs, LP, LSP, Pairs, RGB20, Token, Liquid};

fn main() {
    let mut swap = Bitswap::new();
    let pairs = Pairs::new(1000, 1000, 1000, 1000);
    swap.add_swap(Box::new(pairs));
    swap.add_swap(Box::new(LP::new(1000, 1000, 1000, 1000)));
    swap.add_swap(Box::new(LSP::new(1000, 1000, 1000, 1000)));
    swap.add_swap(Box::new(RGB20::new(1000, 1000, 1000, 1000)));
    swap.add_
    swap(Box::new(Liquid::new(1000, 1000, 1000, 1000)));
    swap.add_swap(Box::new(Token::new(1000, 1000, 1000, 1000)));
    swap.add_swap(Box::new(Token::new(1000, 1000, 1000, 1000)));
    swap.add_swap(Box::new(Token::new(1000, 1000, 1000, 1000)));
}
impl Swap {
    pub fn swap(&mut self, pair: Box<dyn Swap>) {
        self.swaps.push(pair);
    }
}