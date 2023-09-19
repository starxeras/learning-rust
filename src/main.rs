/*

learning-rust - Learning Rust in Public
Written in 2023 by starxeras | starxeras@gmail.com

To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring 
rights to this software to the public domain worldwide. This software is distributed without any warranty.

You should have received a copy of the CC0 Public Domain Dedication along with this software. 
If not, see <http://creativecommons.org/publicdomain/zero/1.0/>. 

*/

extern crate question;
use rand::prelude::*;
// use question::{Question, Answer};

pub struct Random;

impl Random {
    fn hc128_rng(min: i32, max: i32) -> i32 {
        let mut rng: rand_hc::Hc128Rng = rand_hc::Hc128Rng::from_entropy();
        return rng.gen_range(min..max);
    }
    fn chacha20_rng(min: i32, max: i32) -> i32 {
        let mut rng: rand_chacha::ChaCha20Rng = rand_chacha::ChaCha20Rng::from_entropy();
        return rng.gen_range(min..max);
    }
}



fn main() {
    
    loop {
        
        let min = Random::hc128_rng(1, 1000);
        let max = Random::hc128_rng(1, 1000);

        if max <= min {
            continue;
        }

        let mut count = Random::hc128_rng(min, max);
        while count <= 1000 {
            println!("Current count: {}", count);
            println!("---------------");
            println!("Hc128Rng: {}", Random::hc128_rng(min ,max));
            println!("Chacha20Rng: {}\n-----------------", Random::chacha20_rng(min ,max));
            count += 1;
        }

    }

}
