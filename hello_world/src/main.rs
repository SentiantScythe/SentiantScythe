
use std::io::{self, Read};
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    print!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "nice to meet you";
    io::stdin().read_line(Buf: &mut name) Result <usize, Error>
    .expect(msg: "Didnt recieve input");
    println!("Hello{}!!! {}", name.trim_end(), greeting);
}
