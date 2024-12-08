use std::fs;

fn main() {
    println!("Day2");
    let input = fs::read_to_string("./data/input2_1.txt").unwrap();
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let scann_report: usize = reports
        .iter()
        .map(|report| {
            let first_result = scan_line(report);

            if first_result == 1 {
                first_result
            } else {
                for i in 0..report.len() {
                    let mut temp_report = report.clone();
                    let _ = temp_report.remove(i);
                    if scan_line(&temp_report) == 1 {
                        return 1;
                    }
                }
                return 0;
            }
        })
        .sum(); //count the safe evaluations
    println!("{:?}", scann_report);
}

fn scan_line(report: &Vec<usize>) -> usize {
    let win_iter = report.windows(2);
    let init: isize = report[1] as isize - report[0] as isize;
    let increasing = match init {
        x if x < 0 => false,
        x if x > 0 => true,
        _ => return 0, // not increasing, not decreasing => unsafe
    };

    let line_scan: usize = win_iter
        .map(|w| {
            let r: isize = w[1] as isize - w[0] as isize;
            if (r > 0) == increasing && r.abs() > 0 && r.abs() < 4 {
                0 // safe
            } else {
                1 //unsafe
            }
        })
        .sum();
    if line_scan > 0 {
        0 // unsafe
    } else {
        1 //safe
    }
}
