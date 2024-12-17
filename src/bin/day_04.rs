use std::{fmt::Debug, fs, isize};

#[derive(Debug)]
pub struct Grid<T: Debug + PartialEq + Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct Pos {
    x: isize,
    y: isize,
}

impl Grid<char> {
    pub fn search_part2(&self, value: char) {
        let _r: Vec<_> = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if *c == value {
                    let (x, y) = (
                        i as isize % self.width as isize,
                        (i as isize / self.width as isize).abs(),
                    );
                    let cross_one = vec![
                        self.get_from_pos(Pos { x: x - 1, y: y - 1 }),
                        Some(&value),
                        self.get_from_pos(Pos { x: x + 1, y: y + 1 }),
                    ];
                    let cross_two = vec![
                        self.get_from_pos(Pos { x: x - 1, y: y + 1 }),
                        Some(&value),
                        self.get_from_pos(Pos { x: x + 1, y: y - 1 }),
                    ];

                    if cross_one.iter().any(|&item| item.is_none())
                        || cross_two.iter().any(|&item| item.is_none())
                    {
                        None
                    } else {
                        let c1: Vec<_> = cross_one
                            .into_iter()
                            .map(|item| item.cloned().unwrap())
                            .collect();
                        let c2: Vec<_> = cross_two
                            .into_iter()
                            .map(|item| item.cloned().unwrap())
                            .collect();

                        let s1 = vec!['M', 'A', 'S'];
                        let s2 = vec!['S', 'A', 'M'];
                        if (c1 == s1 || c1 == s2) && (c2 == s1 || c2 == s2) {
                            Some(true)
                        } else {
                            None
                        }
                    }
                    //Some(())
                } else {
                    None
                }
            })
            .collect();

        println!("part2: {}", _r.len());
    }
}
impl<T: Debug + PartialEq + Clone> Grid<T> {
    pub fn new(w: usize, h: usize, data: Vec<T>) -> Self {
        if w * h != data.len() {
            panic!("data had wrong size");
        }
        Grid {
            width: w,
            height: h,
            data,
        }
    }

    pub fn get_from_pos(&self, pos: Pos) -> Option<&T> {
        if pos.x >= self.width as isize || pos.x < 0 || pos.y >= self.height as isize || pos.y < 0 {
            None
        } else {
            Some(&self.data[pos.y as usize * self.width + pos.x as usize])
        }
    }

    pub fn get_lines(&self, pos: Pos, l: isize) -> Vec<Vec<T>> {
        let (
            mut line1,
            mut line2,
            mut line3,
            mut line4,
            mut line5,
            mut line6,
            mut line7,
            mut line8,
        ) = (
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        //let max_y = pos.y + l - 1;

        for i in 0..l {
            line1.push(self.get_from_pos(Pos {
                x: pos.x - i,
                y: pos.y - i,
            }));
            line2.push(self.get_from_pos(Pos {
                x: pos.x - i,
                y: pos.y,
            }));
            line3.push(self.get_from_pos(Pos {
                x: pos.x - i,
                y: pos.y + i,
            }));
            line4.push(self.get_from_pos(Pos {
                x: pos.x,
                y: pos.y - i,
            }));

            line5.push(self.get_from_pos(Pos {
                x: pos.x + i,
                y: pos.y + i,
            }));
            line6.push(self.get_from_pos(Pos {
                x: pos.x + i,
                y: pos.y,
            }));
            line7.push(self.get_from_pos(Pos {
                x: pos.x + i,
                y: pos.y - i,
            }));
            line8.push(self.get_from_pos(Pos {
                x: pos.x,
                y: pos.y + i,
            }));
        }

        let lines: Vec<Vec<Option<&T>>> =
            vec![line1, line2, line3, line4, line5, line6, line7, line8];
        //println!("{:?}", lines);
        let r: Vec<Vec<T>> = lines
            .into_iter()
            .filter(|line| !line.iter().any(|&item| item.is_none()))
            .map(|line| {
                line.into_iter()
                    .map(|value| value.cloned().unwrap())
                    .collect()
            })
            .collect();
        r
    }
    pub fn search_part1(&self, values: Vec<T>) {
        let l = values.len();
        let _r: Vec<_> = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if *c == values[0] {
                    let (x, y) = (i % self.width, (i as isize / self.width as isize).abs());
                    let star = self.get_lines(
                        Pos {
                            x: x as isize,
                            y: y as isize,
                        },
                        l as isize,
                    );

                    let r: Vec<Vec<_>> = star.into_iter().filter(|line| *line == values).collect();
                    Some(r)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        println!("r:{:?}", _r.len());
    }
}

fn main() {
    //let input = fs::read_to_string("./data/input_test_4_1.txt").unwrap();
    let input = fs::read_to_string("./data/input4_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let w = lines[0].len();
    let h = lines.len();

    println!("{}, {}", w, h);
    let data: Vec<_> = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .flatten()
        .collect();
    // println!("{:?}", data);
    let g = Grid::new(w, h, data);

    g.search_part1(vec!['X', 'M', 'A', 'S']);
    g.search_part2('A');
}
