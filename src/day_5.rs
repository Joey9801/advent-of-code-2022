struct Move {
    quantity: usize,
    source: usize,
    sink: usize,
}

pub struct ParsedInput {
    // The top element of the nth stack is the last element of stacks[n]
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl std::convert::AsRef<ParsedInput> for ParsedInput {
    fn as_ref(&self) -> &ParsedInput {
        self
    }
}

fn parse_move(m: &str) -> Move {
    let mut parts = m.split_whitespace();
    assert_eq!(parts.next(), Some("move"));
    let quantity = parts.next().unwrap().parse().unwrap();
    assert_eq!(parts.next(), Some("from"));
    let source = parts.next().unwrap().parse().unwrap();
    assert_eq!(parts.next(), Some("to"));
    let sink = parts.next().unwrap().parse().unwrap();

    Move {
        quantity,
        source,
        sink,
    }
}

fn parse_stacks(stack_lines: Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks = stack_lines
        .last()
        .unwrap()
        .split_whitespace()
        .count();
    
    let mut stacks = vec![Vec::new(); num_stacks];
    
    for line in stack_lines.iter().rev().skip(1) {
        for stack_idx in 0..num_stacks {
            let loc = stack_idx * 4 + 1;
            if loc >= line.len() {
                break;
            }
            
            let c = line.chars().nth(loc).unwrap();
            if c != ' ' {
                stacks[stack_idx].push(c);
            }
        }
    }

    stacks
}

pub fn parse(input: &str) -> ParsedInput {
    let mut lines = input.lines();

    let mut stack_lines = Vec::new();
    let mut stacks = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            stacks = parse_stacks(stack_lines);
            break;
        }
        
        stack_lines.push(line);
    }

    let mut moves = Vec::new();
    for line in lines {
        moves.push(parse_move(line));
    }
    
    ParsedInput {
        stacks,
        moves,
    }
}

pub fn solve_part_1(input: &ParsedInput) -> String {
    let mut stacks = input.stacks.clone();

    for m in &input.moves {
        for _ in 0..m.quantity {
            let c = stacks[m.source - 1].pop().unwrap();
            stacks[m.sink - 1].push(c);
        }
    }
    
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect()
}

pub fn solve_part_2(input: &ParsedInput) -> String {
    let mut stacks = input.stacks.clone();

    let mut temp = Vec::new();
    for m in &input.moves {
        for _ in 0..m.quantity {
            let c = stacks[m.source - 1].pop().unwrap();
            temp.push(c);
        }
        for c in temp.drain(..).rev() {
            stacks[m.sink - 1].push(c);
        }
    }
    
    stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect()
}