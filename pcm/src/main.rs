// Packages, Crates, & Modules

// use std::io::{self, Write};
// use http;

mod game;
pub use crate::game::actors::{Player};

fn main() {
    println!("Hello, world!");

    if true {
        println!("Hello again.");
    }

    Player::name();
}
