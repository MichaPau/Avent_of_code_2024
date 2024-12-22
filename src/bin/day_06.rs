use std::fs;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
#[derive(Debug)]
pub struct Grid {
    pub array: Vec<char>,
    pub width: usize,
    pub height: usize,
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
}
fn main() {
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
        let mut dir = Direction::try_from(*g.get_value_from_point(walker_pos).unwrap()).unwrap();
        let walker_index = g.get_index_from_pos(walker_pos);
        g.array[walker_index] = 'X';
        let steps = g.walk(&mut walker_pos, &mut dir);
        println!("walker steps: {}", steps);
        //right answer: 4758
    } else {
        println!("walker not found");
    }
}
