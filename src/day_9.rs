use std::collections::HashSet;

use crate::util::{Dir, Vec2};

pub struct RopeMove {
    dir: Dir,
    mag: i32,
}

fn new_tail_pos(head: Vec2, tail: Vec2) -> Vec2 {
    // Vector from head -> tail 
    let diff = tail - head;
    
    if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
        // Tail is already in a legal position, no modification
        return tail;
    }
    
    let new_diff = if diff.x == 0 || diff.y == 0 {
        Vec2 { x: diff.x.signum(), y: diff.y.signum() }
    } else {
        let step = Vec2 { x: diff.x.signum(), y: diff.y.signum() };
        diff - step
    };
    
    head + new_diff
}

pub fn parse(input: &str) -> Vec<RopeMove> {
    input
        .lines()
        .map(|line| {
            let (dir, mag) = {
                let mut parts = line.split_whitespace();
                (parts.next().unwrap(), parts.next().unwrap())
            };

            let dir = match dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("Invalid dir char"),
            };
            
            let mag = mag.parse().expect("Invalid magnitude");
            
            RopeMove { dir, mag }
        })
        .collect()
}

fn solve(input: &[RopeMove], rope_len: usize) -> usize {
    assert!(rope_len > 0);
    let mut rope = vec![Vec2::zero(); rope_len];
    
    let mut tail_positions = HashSet::new();
    tail_positions.insert(Vec2::zero());
    
    for m in input {
        for _ in 0..m.mag {
            rope[0] = rope[0] + m.dir.to_vec2();

            for i in 1..rope_len {
                rope[i] = new_tail_pos(rope[i - 1], rope[i]);
            }
            tail_positions.insert(*rope.last().unwrap());
        }
    }

    tail_positions.len()
}

pub fn solve_part_1(input: &[RopeMove]) -> usize {
    solve(input, 2)
}

pub fn solve_part_2(input: &[RopeMove]) -> usize {
    solve(input, 10)
}