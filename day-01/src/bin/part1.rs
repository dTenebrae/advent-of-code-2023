use std::fs;

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
    let result: Result<u32, std::num::ParseIntError> =
        format!("{}{}", current_chars[start_ptr], current_chars[end_ptr]).parse();
    match result {
        Ok(num) => return num,
        Err(_) => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_digits_1() {
        let result = get_num("1abc2");
        assert_eq!(result, 12);
    }

    #[test]
    fn multiple_digits_2() {
        let result = get_num("pqr3stu8vwx");
        assert_eq!(result, 38);
    }

    #[test]
    fn multiple_digits_3() {
        let result = get_num("a1b2c3d4e5f");
        assert_eq!(result, 15);
    }

    #[test]
    fn one_digit() {
        let result = get_num("treb7uchet");
        assert_eq!(result, 77);
    }

    #[test]
    fn no_digits() {
        let result = get_num("lasslsdkjfxc[jd");
        assert_eq!(result, 0);
    }

    #[test]
    fn sum_digits() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result: u32 = input
            .lines()
            .into_iter()
            .map(|line: &str| get_num(line))
            .sum();
        assert_eq!(result, 142);
    }
}
