use ndarray::prelude::*;
use std::fs;

const CHR: i32 = -1;
const DOT: i32 = -2;
const STAR: i32 = -3;

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");
    let arr = matrix_from_string(&contents);
    println!("{}", sum_adjacents(arr));
}

fn sum_adjacents(arr: ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>>) -> i32 {
    let mut summator: i32 = 0;
    let mut current_digit: Vec<i32> = vec![];
    let mut adjacent_flag = false;

    for x in 0..arr.shape()[1] {
        for y in 0..arr.shape()[0]{
            if arr[[x, y]] >= 0 {
                current_digit.push(arr[[x, y]]);
                let window = get_window(x, y, &arr);
                if is_adjacent(&window) {
                    adjacent_flag = true;
                }
            } else {
                if adjacent_flag {
                     summator += vec_concat(&current_digit);
                    adjacent_flag = false;
                }
            current_digit.clear();
            }
        }
    }
    summator
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

fn vec_concat(vec: &[i32]) -> i32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}