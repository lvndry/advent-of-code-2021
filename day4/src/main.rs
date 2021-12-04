use ansi_term::Colour::Red;
use std::fs;
use std::path::Path;

#[derive(Default, Debug, Copy, Clone)]
struct Cell {
    value: i32,
    marked: bool,
}

impl Cell {
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Copy, Clone)]
struct Board {
    cells: [[Cell; 5]; 5],
}

impl Board {
    fn from_str(input: &str) -> Self {
        let mut cells = [Default::default(); 25];

        for (i, cell) in input.split_whitespace().enumerate() {
            cells[i] = Cell {
                value: cell.parse::<i32>().unwrap(),
                marked: false,
            };
        }

        Self {
            cells: unsafe { std::mem::transmute::<[Cell; 25], [[Cell; 5]; 5]>(cells) },
        }
    }

    fn mark_number(&mut self, number: i32) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                if self.cells[x][y].value == number {
                    self.cells[x][y].mark();
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        if (0..self.cells.len()).all(|i| self.cells[i][0].marked) {
            return true;
        }

        for i in 0..self.cells.len() {
            if (0..self.cells[i].len()).all(|j| self.cells[i][j].marked) {
                return true;
            }
        }

        false
    }

    fn sum_of_unmarked(&self) -> i32 {
        self.cells.into_iter().fold(0, |acc, line| {
            acc + (line
                .into_iter()
                .filter(|c| !c.marked)
                .map(|c| c.value)
                .sum::<i32>())
        })
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cells.into_iter().for_each(|line| {
            line.into_iter().for_each(|c| {
                if c.marked {
                    write!(f, "{} ", Red.paint(c.value.to_string()));
                } else {
                    write!(f, "{} ", c.value.to_string());
                }
            });
            writeln!(f);
        });
        writeln!(f)
    }
}

fn main() {
    let input = Path::new("./input.txt");
    let content = fs::read_to_string(input).expect("Unable to read file");
    let (numbers, boards) = content.split_once('\n').unwrap();

    let numbers: Vec<i32> = numbers
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = boards.split("\n\n").map(Board::from_str).collect();
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.has_won() {
                println!("{}", board);
                println!("{}", board.sum_of_unmarked() * number);
                return;
            }
        }
    }
}
