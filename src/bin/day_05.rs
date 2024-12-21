use std::{fs, usize};

fn main() {
    let mut rules: Vec<(usize, usize)> = vec![];
    let mut page_numbers: Vec<Vec<usize>> = vec![];

    let input = fs::read_to_string("./data/input5_1.txt").unwrap();
    //let input = fs::read_to_string("./data/input_test_5_1.txt").unwrap();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (r1, r2): (usize, usize) = line
            .split_once("|")
            .map(|s| (s.0.parse().unwrap(), s.1.parse().unwrap()))
            .unwrap();
        rules.push((r1, r2));
    }

    while let Some(line) = lines.next() {
        let pages: Vec<usize> = line.split(",").map(|item| item.parse().unwrap()).collect();
        page_numbers.push(pages);
    }

    let mut wrong_pages: Vec<Vec<usize>> = vec![];
    let r: usize = page_numbers
        .iter()
        .filter(|&page| {
            page.windows(2).all(|p| {
                let t = rules.iter().any(|r| *r == (p[1], p[0]));
                if t {
                    let new_page = page.clone();
                    wrong_pages.push(new_page);
                }
                !t
            })
        })
        .map(|page| page[(page.len() - 1) / 2])
        .sum();
    //.collect();

    let wr: usize = wrong_pages
        .iter_mut()
        .map(|page| {
            loop {
                let mut all_good = true;
                for i in 0..page.len() - 1 {
                    for rule in rules.iter() {
                        if *rule == (page[i + 1], page[i]) {
                            all_good = false;
                            page.swap(i, i + 1);
                            break;
                        }
                    }
                }

                if all_good {
                    break;
                }
            }

            page.clone()
        })
        .map(|page| page[(page.len() - 1) / 2])
        .sum();

    println!("sum 1: {}", r);
    println!("wrongs: {:?}", wr);
}
