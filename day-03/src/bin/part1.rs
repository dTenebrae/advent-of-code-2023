use ndarray::prelude::*;
use std::fs;

const DOT: i32 = -2;
const CHR: i32 = -1;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    let arr = matrix_from_string(&contents);
    println!("{:?}", arr);

    let window = get_window(1, 3, &arr);
    println!("{:?}", window);
    println!("{}", is_adjacent(&window));

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
    let mut from_x: usize;
    let mut to_x: usize;
    let mut from_y: usize;
    let mut to_y: usize;

    match x.cmp(&0) {
        std::cmp::Ordering::Equal => {
            from_x = x;
            to_x = x + 1;
        },
        std::cmp::Ordering::Greater => {
            from_x = x - 1;
            to_x = x + 1;
        },
        std::cmp::Ordering::Less => {
            panic!("Somehow usize is negative");
        }
    };

    match y.cmp(&0) {
        std::cmp::Ordering::Equal => {
            from_y = y;
            to_y = y + 1;
        },
        std::cmp::Ordering::Greater => {
            from_y = y - 1;
            to_y = y + 1;
        },
        std::cmp::Ordering::Less => {
            panic!("Somehow usize is negative");
        }

    }

    let max_x =  arr.shape()[0] - 1;
    let max_y =  arr.shape()[1] - 1;

    match x.cmp(&max_x) {
        std::cmp::Ordering::Equal => {
            from_x = x - 1;
            to_x = x;
        },
        std::cmp::Ordering::Greater => {
            panic!("should not be greater than 139");
        },
        std::cmp::Ordering::Less => {}
    };

    match y.cmp(&max_y) {
        std::cmp::Ordering::Equal => {
            from_y = y - 1;
            to_y = y;
        },
        std::cmp::Ordering::Greater => {
            panic!("should not be greater than 139");
        },
        std::cmp::Ordering::Less => {}

    }
    let result = arr.slice(s![
        from_x..=to_x,
        from_y..=to_y
    ]);
    result
}

fn is_adjacent(window: &ArrayBase<ndarray::ViewRepr<&i32>, Dim<[usize; 2]>>) -> bool {
    for cell in window {
        if *cell == CHR {
            return true
        }
    }
    false
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
