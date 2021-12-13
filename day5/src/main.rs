use std::fmt;
use std::fs;
use std::ops::{Index, IndexMut};
use std::path::Path;

#[derive(Debug)]
struct Line {
    x1: u16,
    x2: u16,
    y1: u16,
    y2: u16,
}

impl Line {
    fn from_str(line: &str) -> Result<Self, ()> {
        let mut coords = line.split(|c| c == ',' || c == ' ');
        let x1: u16 = coords.next().unwrap().parse().unwrap();
        let y1: u16 = coords.next().unwrap().parse().unwrap();
        let _ = coords.next();
        let x2: u16 = coords.next().unwrap().parse().unwrap();
        let y2: u16 = coords.next().unwrap().parse().unwrap();

        if x1 == x2 || y1 == y2 {
            Ok(Self {
                x1: x1.min(x2),
                y1: y1.min(y2),
                x2: x1.max(x2),
                y2: y1.max(y2),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Copy)]
enum GridPoint {
    Empty,
    Occupied,
    OverStepped,
}

struct Grid {
    points: Vec<GridPoint>,
    count: u16,
}

impl Grid {
    fn new() -> Self {
        Self {
            points: vec![GridPoint::Empty; 1024 * 1024],
            count: 0,
        }
    }

    fn insert_line(&mut self, line: Line) {
        if line.x1 == line.x2 {
            for y in line.y1..=line.y2 {
                self.insert_point(line.x1, y);
            }
        } else {
            for x in line.x1..=line.x2 {
                self.insert_point(x, line.y1);
            }
        }
    }

    fn insert_point(&mut self, x: u16, y: u16) {
        match self[(x, y)] {
            GridPoint::Empty => self[(x, y)] = GridPoint::Occupied,
            GridPoint::Occupied => {
                self[(x, y)] = GridPoint::OverStepped;
                self.count += 1;
            }
            _ => {}
        }
    }
}

impl Index<(u16, u16)> for Grid {
    type Output = GridPoint;
    fn index(&self, (x, y): (u16, u16)) -> &Self::Output {
        &self.points[1024 * (x as usize) + (y as usize)]
    }
}

impl IndexMut<(u16, u16)> for Grid {
    fn index_mut(&mut self, (x, y): (u16, u16)) -> &mut Self::Output {
        &mut self.points[1024 * (x as usize) + (y as usize)]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(1024 + 1);
        f.write_str("\n")?;
        for x in 0..1024 {
            s.clear();
            for y in 0..1024 {
                match self[(x as u16, y as u16)] {
                    GridPoint::Empty => s.push('.'),
                    GridPoint::Occupied => s.push('X'),
                    GridPoint::OverStepped => s.push('#'),
                }
            }
            s.push('\n');
            f.write_str(&s)?;
        }
        Ok(())
    }
}

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    let lines = content.split('\n');
    let mut grid = Grid::new();
    for line in lines {
        if let Ok(line) = Line::from_str(line) {
            grid.insert_line(line);
        }
    }

    println!("{:?}", grid);
    println!("{}", grid.count);
}
