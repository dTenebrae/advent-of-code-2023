use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");
    println!("{}", contants);
}