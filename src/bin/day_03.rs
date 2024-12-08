use std::fs;

use regex::Regex;

fn main() {
    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"; // -> 161
    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = fs::read_to_string("./data/input3_1.txt").unwrap();

    part_01(&input);
    part_02(&input);
}

fn part_01(input: &str) {
    let re = Regex::new(r"mul\((?<n1>[0-9]*),(?<n2>[0-9]*)\)").unwrap();
    let result: usize = re
        .captures_iter(&input)
        .map(|caps| {
            let n1: usize = caps.name("n1").unwrap().as_str().parse().unwrap();
            let n2: usize = caps.name("n2").unwrap().as_str().parse().unwrap();
            n1 * n2
        })
        .sum();

    println!("part 1 result: {}", result);
}

fn part_02(input: &str) {
    let re = Regex::new(r"(mul\((?<n1>[0-9]*),(?<n2>[0-9]*)\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let result: usize = re
        .captures_iter(&input)
        .map(|caps| match caps.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            s if s.starts_with("mul(") => {
                if enabled {
                    let n1: usize = caps.name("n1").unwrap().as_str().parse().unwrap();
                    let n2: usize = caps.name("n2").unwrap().as_str().parse().unwrap();
                    n1 * n2
                } else {
                    0
                }
            }
            s => {
                println!("{}", s);
                0
            }
        })
        .sum();

    //println!("{:?}", m);
    println!("part 2 result: {}", result);
}
