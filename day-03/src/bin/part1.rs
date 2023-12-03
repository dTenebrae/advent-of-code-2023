use ndarray::prelude::*;
use std::{fs, result};

const DOT: i32 = -2;
const CHR: i32 = -1;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    let arr = matrix_from_string(&contents);
    println!("{:?}", arr);

    let window = get_window(0, 0, &arr);
    println!("{:?}", window);

    // let result: u32 = contents
    //     .lines()
    //     .into_iter()
    //     .map(|line: &str| game_checker(line))
    //     .sum();
    // println!("{}", result)
}

fn matrix_from_string(input: &String) -> ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>> {
    let ax_x = input.lines().next().unwrap().len();
    let ax_y = input.match_indices('\n').into_iter().count();

    let mut arr = Array::<i32, _>::zeros((ax_x, ax_y).f());

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                arr[[y, x]] = chr.to_digit(10).unwrap() as i32;
            } else if chr == '.' {
                arr[[y, x]] = DOT;
            } else {
                arr[[y, x]] = CHR;
            }
        }
    }

    arr
}

fn get_window<'a>(
    x: usize,
    y: usize,
    arr: &'a ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>>,
) -> ArrayBase<ndarray::ViewRepr<&'a i32>, Dim<[usize; 2]>> {
    // TODO make this boundries from array dim
    let min_value = 1;
    let max_value = 138;
    let trimmed_x = std::cmp::max(std::cmp::min(x, max_value), min_value);
    let trimmed_y = std::cmp::max(std::cmp::min(y, max_value), min_value);
    println!("{}: {}", trimmed_x, trimmed_y);
    let result = arr.slice(s![
        trimmed_x - 1..=trimmed_x + 1,
        trimmed_y - 1..=trimmed_y + 1
    ]);
    result
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
        assert_eq!(9, 9);
    }
}
