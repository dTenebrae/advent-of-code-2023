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
    counter
}

fn read_values<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    line.trim().split_whitespace().map(|word| word.parse()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result: u32 = input
            .lines()
            .into_iter()
            .map(|line| card_checker(line))
            .sum();
        assert_eq!(result, 30);
    }
}







