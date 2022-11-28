#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     let greeting = "Nice to meet you";
//     io::stdin().read_line(&mut name).expect("Didn't recieve Input"); 
//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

// fn main() {
//     let age = 8;
//     if(age >= 1) && (age <= 18){
//         println!("Important Birthday");
//     } else if (age == 21) && (age == 50){
//         println!("Important Birthday");
//     } else {
//         println!("Not an Important Birthday");
//     }
// }


// fn main() {
//     let age2 = 8;
//     match age2 {
//         1..=18 => println!("Important Birthday"),
//         21 | 50 => println!("Important Birthday"),
//         65..=i32::MAX => println!("Important Birthday"),
//         _ => println!("Not Important")
//     };
// }

fn main() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You have earn the right to vote"),
    };
}