use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Can't go that direction"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
impl Direction {
    pub fn turn(&mut self) {
        let d = self.clone();
        *self = Direction::from(((d as u8) + 1) % 4);
        //Direction::from(((dir as u8) + 1) % 4)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
#[derive(Debug)]
pub struct Grid {
    pub array: Vec<char>,
    pub width: usize,
    pub height: usize,
    pub dir_map: HashMap<usize, Direction>,
}

impl Grid {
    pub fn new(w: usize, h: usize, data: Vec<char>) -> Self {
        if w * h != data.len() {
            panic!("data has wrong size");
        }
        Grid {
            width: w,
            height: h,
            array: data,
            dir_map: HashMap::new(),
        }
    }

    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    self.get_value_from_point(Point {
                        x: x as isize,
                        y: y as isize
                    })
                    .unwrap()
                )
            }
            print!("\n");
        }
    }
    pub fn get_value_from_point(&self, p: Point) -> Option<&char> {
        if p.x < 0 || p.x >= self.width as isize || p.y < 0 || p.y >= self.height as isize {
            None
        } else {
            Some(&self.array[p.y as usize * self.width + p.x as usize])
        }
    }

    pub fn get_pos_from_index(&self, i: usize) -> Point {
        let y = i / self.width;
        let x = i - (y * self.width);

        Point {
            x: x as isize,
            y: y as isize,
        }
    }

    pub fn get_index_from_pos(&self, pos: Point) -> usize {
        pos.y as usize * self.width + pos.x as usize
    }
    pub fn get_walker_pos(&self) -> Option<Point> {
        let index = self
            .array
            .iter()
            .position(|entry| Direction::try_from(*entry).is_ok());

        match index {
            Some(index) => Some(self.get_pos_from_index(index)),
            None => None,
        }
    }

    pub fn collect_possible_obstacles(&self) -> Vec<usize> {
        let r: Vec<usize> = self
            .array
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                if *entry == 'X' {
                    //Some(self.get_pos_from_index(i))
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        r
    }
    pub fn walk(&mut self, from: &mut Point, dir: &mut Direction) -> usize {
        let mut steps = 1;
        loop {
            let check_pos_at = match dir {
                Direction::Up => Point {
                    x: from.x,
                    y: from.y - 1,
                },
                Direction::Down => Point {
                    x: from.x,
                    y: from.y + 1,
                },
                Direction::Left => Point {
                    x: from.x - 1,
                    y: from.y,
                },
                Direction::Right => Point {
                    x: from.x + 1,
                    y: from.y,
                },
            };

            match self.get_value_from_point(check_pos_at) {
                Some('.') => {
                    steps += 1;
                    from.x = check_pos_at.x;
                    from.y = check_pos_at.y;
                    let index = self.get_index_from_pos(*from);
                    self.dir_map.insert(index, dir.clone());
                    self.array[index] = 'X';
                }
                Some('#') => {
                    dir.turn();
                }
                Some('X') => {
                    from.x = check_pos_at.x;
                    from.y = check_pos_at.y;
                }
                None => break,
                Some(x) => {
                    println!("what: {}", x);
                    panic!("walking delirium..")
                }
            }

            if steps > self.array.len() {
                panic!("walking eternally");
            }
        }

        steps
    }
    pub fn obstacle_walk(&mut self, from: &mut Point, dir: &mut Direction) -> bool {
        let mut obstacle_bums: HashMap<(usize, Direction), usize> = HashMap::new();
        let endless_loop;
        loop {
            let check_pos_at = match dir {
                Direction::Up => Point {
                    x: from.x,
                    y: from.y - 1,
                },
                Direction::Down => Point {
                    x: from.x,
                    y: from.y + 1,
                },
                Direction::Left => Point {
                    x: from.x - 1,
                    y: from.y,
                },
                Direction::Right => Point {
                    x: from.x + 1,
                    y: from.y,
                },
            };

            let value = self.get_value_from_point(check_pos_at);

            match value {
                Some('.') => {
                    from.x = check_pos_at.x;
                    from.y = check_pos_at.y;
                }
                Some('#') => {
                    let index = self.get_index_from_pos(check_pos_at);
                    *obstacle_bums.entry((index, *dir)).or_insert(1) += 1;
                    if *obstacle_bums.get(&(index, *dir)).unwrap() > 2 {
                        endless_loop = true;
                        break;
                    }

                    dir.turn();
                }
                Some('X') => {
                    from.x = check_pos_at.x;
                    from.y = check_pos_at.y;
                }
                Some('O') => {
                    let index = self.get_index_from_pos(check_pos_at);
                    *obstacle_bums.entry((index, *dir)).or_insert(1) += 1;
                    if *obstacle_bums.get(&(index, *dir)).unwrap() > 2 {
                        endless_loop = true;
                        break;
                    }
                    dir.turn();
                }
                None => {
                    endless_loop = false;
                    break;
                }
                Some(x) => {
                    println!("what: {}", x);
                    panic!("walking delirium..")
                }
            }
        }

        endless_loop
    }
}
fn main() {
    let start = Instant::now();
    //let input = fs::read_to_string("./data/input_test_6_1.txt").unwrap();
    let input = fs::read_to_string("./data/input6_1.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let w = lines[0].len();
    let h = lines.len();

    let data: Vec<_> = lines
        .into_iter()
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut g = Grid::new(w, h, data);

    if let Some(mut walker_pos) = g.get_walker_pos() {
        let init_walker_pos = Point {
            x: walker_pos.x,
            y: walker_pos.y,
        };
        let init_dir = Direction::try_from(*g.get_value_from_point(walker_pos).unwrap()).unwrap();
        let mut dir = init_dir.clone();
        let walker_index = g.get_index_from_pos(walker_pos);
        g.array[walker_index] = 'X';
        g.dir_map.insert(walker_index, init_dir);
        let steps = g.walk(&mut walker_pos, &mut dir);

        //right answer: 4758
        println!("walker steps: {}", steps);

        g.array[walker_index] = '.';

        //part 2
        let pos_obstacles = g.collect_possible_obstacles();

        let count = pos_obstacles
            .iter()
            .filter_map(|&i| {
                // let mut walker_dir = g.dir_map.get(&i).unwrap().clone();
                // let mut start_pos = g.get_pos_from_index(i);

                let mut new_walker = Point {
                    x: init_walker_pos.x,
                    y: init_walker_pos.y,
                };
                let mut dir = init_dir.clone();
                g.array[i] = 'O';
                let w_r = g.obstacle_walk(&mut new_walker, &mut dir);

                //let w_r = g.obstacle_walk(&mut start_pos, &mut walker_dir);

                g.array[i] = 'X';
                if w_r {
                    Some(w_r)
                } else {
                    None
                }
            })
            .count();

        //right answer: 1670
        println!("possible obstacles count: {}", count);
    } else {
        println!("walker not found");
    }

    println!("Measure time: {:?}", start.elapsed().as_millis());
}
