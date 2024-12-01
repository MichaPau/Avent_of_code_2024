use std::{collections::HashMap, fs};

fn main() {
    println!("day1 main");
    let input = fs::read_to_string("./data/input1_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().map(|item| item).collect();
    let l = lines.len();
    let mut left_list: Vec<usize> = Vec::with_capacity(l);
    let mut right_list: Vec<usize> = Vec::with_capacity(l);

    for line in lines {
        if let Some((left, right)) = line.split_once("   ") {
            left_list.push(left.parse().unwrap());
            right_list.push(right.parse().unwrap());
        } else {
            panic!("split error");
        }
    }

    //_part_1_result(&mut left_list, &mut right_list);
    _part_2_result(left_list, right_list);
}

fn _part_2_result(left_list: Vec<usize>, right_list: Vec<usize>) {
    let mut right_map: HashMap<usize, usize> = HashMap::new();
    for i in right_list {
        *right_map.entry(i).or_default() += 1;
    }

    let similarity_sum: usize = left_list
        .into_iter()
        .map(|i| {
            if let Some(c) = right_map.get(&i) {
                c * i
            } else {
                0
            }
        })
        .sum();

    println!("Similarity sum: {:?}", similarity_sum);
}
fn _part_1_result(left_list: &mut Vec<usize>, right_list: &mut Vec<usize>) {
    left_list.sort();
    right_list.sort();

    let zip = left_list.iter().zip(right_list.iter());
    let total: usize = zip.map(|item| item.0.abs_diff(*item.1)).sum();
    println!("Total: {}", total);
}
