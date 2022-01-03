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

impl std::cmp::PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.marked == other.marked
    }
}

#[derive(Debug, Clone, Copy)]
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
        for i in 0..self.cells.len() {
            if (0..self.cells.len()).all(|j| self.cells[i][j].marked) {
                return true;
            }
        }

        for i in 0..self.cells.len() {
            if (0..self.cells.len()).all(|j| self.cells[j][i].marked) {
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

impl std::cmp::PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                if self.cells[x][y] != other.cells[x][y] {
                    return false;
                }
            }
        }
        true
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
    let boards: Vec<Board> = boards.split("\n\n").map(Board::from_str).collect();

    part_1(numbers.clone(), boards.clone());
    part_2(&numbers.clone(), boards.clone());
}

fn part_1(numbers: Vec<i32>, boards: Vec<Board>) {
    let mut boards = boards;
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

fn part_2(numbers: &[i32], boards: Vec<Board>) {
    let mut boards = boards;
    for (_, number) in numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark_number(*number);
        }
        if boards.len() == 1 && boards[0].has_won() {
            let board = boards.remove(0);
            println!("{}", number);
            println!("Last board:\n{}", board);
            println!("{}", board.sum_of_unmarked() * number);
            return;
        }
        boards.retain(|b| !b.has_won())
    }
}
