use std::{fs, str::FromStr};

struct RangeMap {
    src_begin: i64,
    dest_begin: i64,
    range: i64,
}

trait Contains {
    fn contains(&self, num: &i64) -> bool;
    fn dest_num(&self, num: &i64) -> i64;
}

impl Contains for RangeMap {
    fn contains(&self, num: &i64) -> bool {
        self.src_begin <= *num && *num < (self.src_begin + self.range)
    }

    fn dest_num(&self, num: &i64) -> i64 {
        (self.dest_begin - self.src_begin)  + *num
    }
}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    println!("{}", map_seeds(contents));
}

fn map_seeds(contents: String) -> i64 {
    let mut content_iter = contents.lines();
    let seeds = content_iter.next().unwrap();
    let mut seed_vec = read_seeds(seeds);
    let mut read_flag: bool = false;
    let mut range_vec: Vec<RangeMap> = vec![];

    for line in content_iter {

        if line.contains("map:") {
            read_flag = true;
            range_vec.clear();
            continue;
        } else if line.is_empty() {
            if read_flag {
                read_flag = false;
                for num in seed_vec.iter_mut() {
                    for rng in &range_vec {
                        if rng.contains(num) {
                            *num = rng.dest_num(num);
                            break;
                        }
                    }

                }                
            }
            continue;
        }

        if read_flag {
            let tmp_vec: Vec<i64> = read_values(line).unwrap();
            range_vec.push(RangeMap {
                src_begin: tmp_vec[1],
                dest_begin: tmp_vec[0],
                range: tmp_vec[2],
            });

        }
    }

    *seed_vec.iter().min().unwrap()
}

fn read_seeds(line: &str) -> Vec<i64> {
    let mut seeds_iter = line.split(":");
    let _ = seeds_iter.next().unwrap();

    let seeds = seeds_iter.next().unwrap();
    read_values::<i64>(seeds).unwrap()
}

fn read_values<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    line.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn initial_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = map_seeds(input.to_owned());
        assert_eq!(result, 35);
    }
}
