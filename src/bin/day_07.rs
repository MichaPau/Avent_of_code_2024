use itertools::Itertools;
use std::{fs, time::Instant};

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
    if n == 1 {
        operations.iter().map(|s| s.to_string()).collect()
    } else {
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
}
fn main() {
    let mut start = Instant::now();
    let input = fs::read_to_string("./data/input7_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    //let lines: Vec<&str> = vec!["12: 2 3 4"];

    //part1
    calculate(lines.clone(), vec!["+", "*"]);
    println!("part1 time: {}", start.elapsed().as_millis());
    start = Instant::now();
    //part2
    calculate(lines, vec!["+", "*", "|"]);

    //over 3 secs slow
    println!("part2 time: {}", start.elapsed().as_millis());
}

fn calculate(lines: Vec<&str>, op_vec: Vec<&str>) {
    let result: usize = lines
        .into_iter()
        .filter_map(|line| {
            let (_value, numbers) = parse_line(line);
            let combinations = _create_op_comps(numbers.len() - 1, op_vec.clone());
            let good = combinations.into_iter().any(|v| {
                //println!("check: {:?}, {:?}", v, numbers);
                let start = numbers[0];
                let r = numbers
                    .iter()
                    .enumerate()
                    .skip(1)
                    .fold(start, |mut acc, n| match v.chars().nth(n.0 - 1) {
                        Some('+') => acc + n.1,
                        Some('*') => acc * n.1,
                        Some('|') => {
                            let t = format!("{}{}", acc, n.1).parse().unwrap();
                            acc = t;
                            acc
                        }
                        _ => panic!("wrong operator at: {}", n.0 - 1),
                    });

                r == _value
            });

            if good {
                //println!("r:{}", _value);
                Some(_value)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", result);
}

//that was wrong ....
// fn _part2(lines: Vec<&str>) {
//     let _result: Vec<usize> = lines
//         .into_iter()
//         .filter_map(|line| {
//             let (_value, numbers) = parse_line(line);
//             let combinations = _create_op_comps(numbers.len() - 1, vec!["+", "*", "|"]);
//             let mut good = false;
//             for comb in combinations {
//                 let mut n2: Vec<usize> = vec![numbers[0]];
//                 for (i, c) in comb.chars().into_iter().enumerate() {
//                     match c {
//                         '|' => {
//                             let end_index = n2.len() - 1;
//                             let t: usize = format!("{}{}", n2[end_index], numbers[i + 1])
//                                 .parse()
//                                 .unwrap();
//                             n2[end_index] = t;
//                         }
//                         _ => n2.push(numbers[i + 1]),
//                     }
//                 }
//                 let new_comb = comb.replace("|", "");
//                 let start = n2[0];
//                 let r = n2.iter().enumerate().skip(1).fold(start, |acc, n| {
//                     match new_comb.chars().nth(n.0 - 1) {
//                         Some('+') => acc + n.1,
//                         Some('*') => acc * n.1,
//                         _ => panic!("wrong operator at: {}", n.0 - 1),
//                     }
//                 });

//                 if _value == 7290 {
//                     println!(
//                         "new numbers:  {:?}, new comb: {:?}, result: {}",
//                         n2, new_comb, r
//                     );
//                 }

//                 if r == _value {
//                     good = true;
//                     break;
//                 }
//             }
//             if good {
//                 Some(_value)
//             } else {
//                 None
//             }
//         })
//         .collect();

//     println!("result: {:?}", _result);
// }
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
