#![allow(unused)]

use rand::thread_rng;
use rand::Rng;

pub fn modules() {
    println!("Modules notes!")
}

pub fn hello() {
    println!("Hellow from module!");
    let mut thread_rng = thread_rng();
    let random_num: u64 = thread_rng.gen();
    println!("Random number: {}", random_num);
}
