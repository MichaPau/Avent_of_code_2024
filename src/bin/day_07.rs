use itertools::Itertools;
use std::fs;

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let (test_value, number_str) = line.split_once(":").unwrap();
    let value: usize = test_value.parse().unwrap();
    let numbers: Vec<usize> = number_str
        .split(" ")
        .into_iter()
        .filter_map(|n| n.parse().ok())
        .collect();
    (value, numbers)
}

//from https://stackoverflow.com/a/67746758/1456318
fn _create_op_comps(n: usize, operations: Vec<&str>) -> Vec<String> {
    let combinations: Vec<_> = (2..n).fold(
        operations
            .iter()
            .cartesian_product(operations.iter())
            .map(|(&a, &b)| a.to_owned() + b)
            .collect(),
        |acc, _| {
            acc.into_iter()
                .cartesian_product(operations.iter())
                .map(|(a, b)| a.to_owned() + b)
                .collect()
        },
    );

    combinations
}
fn main() {
    let input = fs::read_to_string("./data/input_test_7_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    part1(lines);
}

fn part2(lines: Vec<&str>) {
    unimplemented!()
}
fn part1(lines: Vec<&str>) {
    let result: usize = lines
        .into_iter()
        .filter_map(|line| {
            let (_value, numbers) = parse_line(line);
            let combinations = _create_op_comps(numbers.len() - 1, vec!["+", "*"]);
            let good = combinations.into_iter().any(|v| {
                let start = numbers[0];
                let r = numbers.iter().enumerate().skip(1).fold(start, |acc, n| {
                    match v.chars().nth(n.0 - 1) {
                        Some('+') => acc + n.1,
                        Some('*') => acc * n.1,
                        _ => panic!("wrong operator at: {}", n.0 - 1),
                    }
                });
                r == _value
            });

            if good {
                Some(_value)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", result);
}

#[test]
fn test_skip_fold() {
    let v = vec![1, 2, 3, 4];
    let o = vec!["+", "*", "+"];

    let start = v[0];
    let sum = v.into_iter().enumerate().skip(1).fold(start, |acc, n| {
        println!("info:{} - {} - {}", n.0, n.1, acc);
        match o[n.0 - 1] {
            "+" => acc + n.1,
            "*" => acc * n.1,
            _ => panic!("wrong operator"),
        }
        //acc + n.1
    });

    println!("{}", sum);
}
