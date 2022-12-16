/// --- Day 5: Supply Stacks ---
/// 
/// The expedition can depart as soon as the final supplies have been unloaded from the ships.
/// Supplies are stored in stacks of marked crates, but because the needed supplies are buried
/// under many other crates, the crates need to be rearranged.
/// 
/// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the
/// crates get crushed or fall over, the crane operator will rearrange them in a series of
/// carefully-planned steps. After the crates are rearranged, the desired crates will be at the top
/// of each stack.
/// 
/// The Elves don't want to interrupt the crane operator during this delicate procedure, but they
/// forgot to ask her which crate will end up where, and they want to be ready to unload them as
/// soon as possible so they can embark.
/// 
/// They do, however, have a drawing of the starting stacks of crates and the rearrangement
/// procedure (your puzzle input). For example:
/// 
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
/// 
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
/// 
/// In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on
/// the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are
/// crates M, C, and D. Finally, stack 3 contains a single crate, P.
/// 
/// Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates
/// is moved from one stack to a different stack. In the first step of the above rearrangement
/// procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
/// 
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
/// 
/// In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a
/// time, so the first crate to be moved (D) ends up below the second and third crates:
/// 
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
/// 
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a
/// time, crate C ends up below crate M:
/// 
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
/// 
/// Finally, one crate is moved from stack 1 to stack 2:
/// 
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
/// 
/// The Elves just need to know which crate will end up on top of each stack; in this example, the
/// top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these
/// together and give the Elves the message CMZ.
/// 
/// After the rearrangement procedure completes, what crate ends up on top of each stack?
///
/// --- Part Two ---
/// 
/// As you watch the crane operator expertly rearrange the crates, you notice the process isn't
/// following your prediction.
/// 
/// Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The
/// crane isn't a CrateMover 9000 - it's a CrateMover 9001.
/// 
/// The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather
/// seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
/// 
/// Again considering the example above, the crates begin in the same configuration:
/// 
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
/// 
/// Moving a single crate from stack 2 to stack 1 behaves the same as before:
/// 
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3 
/// 
/// However, the action of moving three crates from stack 1 to stack 3 means that those three moved
/// crates stay in the same order, resulting in this new configuration:
/// 
///         [D]
///         [N]
///     [C] [Z]
///     [M] [P]
///  1   2   3
/// 
/// Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
/// 
///         [D]
///         [N]
/// [C]     [Z]
/// [M]     [P]
///  1   2   3
/// 
/// Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets
/// moved:
/// 
///         [D]
///         [N]
///         [Z]
/// [M] [C] [P]
///  1   2   3
/// 
/// In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
/// 
/// Before the rearrangement process finishes, update your simulation so that the Elves know where
/// they should stand to be ready to unload the final supplies. After the rearrangement procedure
/// completes, what crate ends up on top of each stack?


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