use std::fs;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    for line in contents.lines() {
        println!("{}", line)
    }
    // let result: u32 = contents
    //     .lines()
    //     .into_iter()
    //     .map(|line: &str| game_checker(line))
    //     .sum();
    // println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        todo!();
        assert_eq!(1, 8);
    }
}
