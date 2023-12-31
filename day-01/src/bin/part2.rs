use std::{char::from_digit, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("No file found");

    let result: u32 = contents
        .lines()
        .into_iter()
        .map(|line: &str| get_num(line))
        .sum();
    println!("{}", result)
}

fn get_num(line: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let current_chars: Vec<char> = line.chars().collect();
    let mut start_ptr: usize = 0;
    let mut end_ptr: usize = line.len() - 1;
    for _i in 0..line.len() {
        if !current_chars[start_ptr].is_numeric() {
            start_ptr += 1;
        }
        if !current_chars[end_ptr].is_numeric() {
            end_ptr -= 1;
        }
        if (current_chars[start_ptr].is_numeric() && current_chars[end_ptr].is_numeric())
            || (start_ptr > end_ptr)
        {
            break;
        }
    }

    let mut result: (char, char) = ('a', 'a');

    if current_chars[start_ptr].is_numeric() && current_chars[end_ptr].is_numeric() {
        result = (current_chars[start_ptr], current_chars[end_ptr]);
    } else {
        start_ptr = usize::MAX;
        end_ptr = 0;
    }

    let mut current_min: usize = usize::MAX;
    let mut current_max: usize = 0;

    for (num, str_word) in numbers.into_iter().enumerate() {
        let num_word_idxs: Vec<(usize, &str)> = line.match_indices(str_word).collect();
        if num_word_idxs.is_empty() {
            continue;
        }
        for (pos, _) in [
            num_word_idxs.first().unwrap(),
            num_word_idxs.last().unwrap(),
        ] {
            if *pos < start_ptr && *pos < current_min {
                result.0 = from_digit((num + 1) as u32, 10).unwrap();
                current_min = *pos;
            } else if *pos > end_ptr && *pos > current_max {
                result.1 = from_digit((num + 1) as u32, 10).unwrap();
                current_max = *pos;
            }
        }
    }

    if result.0.is_numeric() && !result.1.is_numeric() {
        result.1 = result.0
    } else if result.1.is_numeric() && !result.0.is_numeric() {
        result.0 = result.1
    }

    let result = format!("{}{}", result.0, result.1).parse();
    match result {
        Ok(num) => return num,
        Err(_) => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_digits_01() {
        let result = get_num("two1nine");
        assert_eq!(result, 29);
    }

    #[test]
    fn multiple_digits_02() {
        let result = get_num("eightwothree");
        assert_eq!(result, 83);
    }

    #[test]
    fn multiple_digits_03() {
        let result = get_num("abcone2threexyz");
        assert_eq!(result, 13);
    }

    #[test]
    fn multiple_digits_04() {
        let result = get_num("xtwone3four");
        assert_eq!(result, 24);
    }

    #[test]
    fn multiple_digits_05() {
        let result = get_num("4nineeightseven2");
        assert_eq!(result, 42);
    }

    #[test]
    fn multiple_digits_06() {
        let result = get_num("zoneight234");
        assert_eq!(result, 14);
    }

    #[test]
    fn multiple_digits_07() {
        let result = get_num("7pqrstsixteen");
        assert_eq!(result, 76);
    }

    #[test]
    fn multiple_digits_08() {
        let result = get_num("1eightwothree");
        assert_eq!(result, 13);
    }

    #[test]
    fn multiple_digits_09() {
        let result = get_num("l9");
        assert_eq!(result, 99);
    }

    #[test]
    fn multiple_digits_10() {
        let result = get_num("429");
        assert_eq!(result, 49);
    }

    #[test]
    fn multiple_digits_11() {
        let result = get_num("one7118");
        assert_eq!(result, 18);
    }

    #[test]
    fn multiple_digits_12() {
        let result = get_num("sixdtzllvpkppvlxhpkfive8sevenmdzpbnlcnfpcltg3");
        assert_eq!(result, 63);
    }

    #[test]
    fn multiple_digits_13() {
        let result = get_num("threerznlrhtkjp23mtflmbrzq395three");
        assert_eq!(result, 33);
    }

    #[test]
    fn one_digit() {
        let result = get_num("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn one_word() {
        let result = get_num("asdfonefsad");
        assert_eq!(result, 11);
    }

    #[test]
    fn no_digits() {
        let result = get_num("lasslsdkjfxc[jd");
        assert_eq!(result, 0);
    }

    #[test]
    fn sum_digits() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result: u32 = input
            .lines()
            .into_iter()
            .map(|line: &str| get_num(line))
            .sum();
        assert_eq!(result, 281);
    }
}
