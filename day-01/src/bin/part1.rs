use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("No file found");

    for line in contents.lines() {
        let current_chars: Vec<char> = line.chars().collect();
        let mut start_ptr: usize = 0;
        let mut end_ptr: usize = line.len() - 1;
        for _i in 0..line.len() {
            println!("{}: {}", current_chars[start_ptr], current_chars[start_ptr]);
            start_ptr += 1;
            end_ptr -= 1;
            if start_ptr > end_ptr {
                break;
            }
            
        }
        break;
    }
}
