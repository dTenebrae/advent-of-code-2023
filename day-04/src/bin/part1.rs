use std::{fs, str::FromStr};

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    let result: u32 = contents
        .lines()
        .into_iter()
        .map(|line: &str| card_checker(line))
        .sum();
    println!("{}", result);
}

fn card_checker(line: &str) -> u32 {
    let mut line_iter = line.split(':');
    let _ = line_iter.next().unwrap();

    let mut all_nums = line_iter.next().unwrap().split('|');
    let win_nums: Vec<u32> = read_values::<u32>(all_nums.next().unwrap()).unwrap();
    let get_nums: Vec<u32> = read_values::<u32>(all_nums.next().unwrap()).unwrap();

    let mut counter = 0;
    for num in win_nums.iter() {
        let exist = get_nums.iter().find(|x| x == &num);
        match exist {
            Some(_) => counter += 1,
            None => continue
        }
    }
    if counter > 0 { u32::pow(2, counter - 1) } else { 0 }
}

fn read_values<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    line.trim().split_whitespace().map(|word| word.parse()).collect()
}

