#[derive(Clone, Copy)]
enum GameOutcome {
    Win,
    Draw,
    Loss
}

impl GameOutcome {
    fn value(self) -> u32 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Draw => 3,
            GameOutcome::Loss => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    
    fn play_against(self, other: Self) -> GameOutcome {
        match (self, other) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => GameOutcome::Win,
            (a, b) if a == b => GameOutcome::Draw,
            _ => GameOutcome::Loss
        }
    }
    
    /// The shape another player should play against this one, in order for that player to receive
    /// the given result.
    fn complement(self, result: GameOutcome) -> Self {
        match (result, self) {
            (GameOutcome::Win, Shape::Rock) => Shape::Paper,
            (GameOutcome::Win, Shape::Paper) => Shape::Scissors,
            (GameOutcome::Win, Shape::Scissors) => Shape::Rock,
            (GameOutcome::Draw, x) => x,
            (GameOutcome::Loss, Shape::Rock) => Shape::Scissors,
            (GameOutcome::Loss, Shape::Paper) => Shape::Rock,
            (GameOutcome::Loss, Shape::Scissors) => Shape::Paper,
        }
    }
}

pub fn parse(input: &str) -> Vec<(u8, u8)> {
    input.lines()
        .map(|line| {
            let line = line.as_bytes();
            (line[0], line[2])
        })
        .collect()
}

pub fn solve_part_1(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|(a, b)| {
            let a = match a {
                b'A' => Shape::Rock,
                b'B' => Shape::Paper,
                b'C' => Shape::Scissors,
                _ => panic!("Invalid char in input")
            };

            let b = match b {
                b'X' => Shape::Rock,
                b'Y' => Shape::Paper,
                b'Z' => Shape::Scissors,
                _ => panic!("Invalid char in input")
            };
            
            b.play_against(a).value() + b.value()
        })
        .sum()
}

pub fn solve_part_2(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|(a, b)| {
            let a = match a {
                b'A' => Shape::Rock,
                b'B' => Shape::Paper,
                b'C' => Shape::Scissors,
                _ => panic!("Invalid char in input")
            };
            
            let result = match b {
                b'X' => GameOutcome::Loss,
                b'Y' => GameOutcome::Draw,
                b'Z' => GameOutcome::Win,
                _ => panic!("Invalid char in input")
            };

            let b = a.complement(result);
            b.value() + result.value()
        })
        .sum()
}