use crate::util::{Vec2, Dir};

#[derive(Debug)]
struct Map<T> {
    size: Vec2,
    values: Vec<T>,
}

impl<T: Copy> Map<T> {
    fn new(size: Vec2, default: T) -> Self {
        let values = vec![default; (size.x * size.y) as usize];
        Self { size, values }
    }

    fn in_bounds(&self, loc: Vec2) -> bool {
        loc.x >= 0 && loc.x < self.size.x && loc.y >= 0 && loc.y < self.size.y
    }

    fn idx(&self, loc: Vec2) -> Option<usize> {
        if self.in_bounds(loc) {
            Some((loc.x + self.size.x * loc.y) as usize)
        } else {
            None
        }
    }

    fn get(&self, loc: Vec2) -> Option<T> {
        Some(self.values[self.idx(loc)?])
    }
    
    #[must_use]
    fn set(&mut self, loc: Vec2, value: T) -> Option<T> {
        let idx = self.idx(loc)?;
        let ret = Some(self.values[idx]);
        self.values[idx] = value;
        ret
    }

}

#[derive(Debug)]
pub struct ParsedInput {
    map: Map<u8>,
    start_loc: Vec2,
    end_loc: Vec2,
}

impl AsRef<ParsedInput> for ParsedInput {
    fn as_ref(&self) -> &ParsedInput {
        self
    }
}

pub fn parse(input: &str) -> ParsedInput {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let values = input
        .bytes()
        .filter(u8::is_ascii_alphabetic)
        .map(|c| match c {
            b'a'..=b'z' => c as u8 - b'a' as u8,
            b'S' => 0,
            b'E' => 25,
            _ => panic!("Invalid char in input: '{c:?}'"),
        })
        .collect();

    let mut start_loc = Vec2::zero();
    let mut end_loc = Vec2::zero();
    for (y, line) in input.lines().enumerate() {
        if let Some(x) = line.bytes().position(|x| x == b'S') {
            start_loc = Vec2 {
                x: x as i32,
                y: y as i32,
            };
        }

        if let Some(x) = line.bytes().position(|x| x == b'E') {
            end_loc = Vec2 {
                x: x as i32,
                y: y as i32,
            };
        }
    }

    let map = Map {
        size: Vec2 {
            x: width as i32,
            y: height as i32,
        },
        values,
    };

    ParsedInput {
        map,
        start_loc,
        end_loc,
    }
}

pub fn solve_part_1(input: &ParsedInput) -> i32 {
    let mut visit_queue = std::collections::VecDeque::new();
    let mut visited = Map::<i32>::new(input.map.size, -1);
    
    visited.set(input.start_loc, 0).unwrap();
    visit_queue.push_front(input.start_loc);
    while let Some(this_loc) = visit_queue.pop_back() {
        let this_height = input.map.get(this_loc).unwrap();
        let this_dist = visited.get(this_loc).unwrap();

        for dir in Dir::ALL {
            let next_loc = this_loc + dir;
            let next_height = match input.map.get(next_loc) {
                Some(h) => h,
                None => continue,
            };
            
            if visited.get(next_loc).unwrap() != -1 {
                // Already in queue
                continue;
            }
            
            if next_height > (this_height + 1) {
                continue;
            }

            if next_loc == input.end_loc {
                return this_dist + 1;
            }

            visited.set(next_loc, this_dist + 1).unwrap();
            visit_queue.push_front(next_loc);
            
        }
    }
    
    panic!("Failed to reach destination");
}

pub fn solve_part_2(input: &ParsedInput) -> i32 {
    let mut visit_queue = std::collections::VecDeque::new();
    let mut visited = Map::<i32>::new(input.map.size, -1);
    
    visited.set(input.end_loc, 0).unwrap();
    visit_queue.push_front(input.end_loc);
    while let Some(this_loc) = visit_queue.pop_back() {
        let this_height = input.map.get(this_loc).unwrap();
        let this_dist = visited.get(this_loc).unwrap();

        for dir in Dir::ALL {
            let next_loc = this_loc + dir;
            let next_height = match input.map.get(next_loc) {
                Some(h) => h,
                None => continue,
            };
            
            if visited.get(next_loc).unwrap() != -1 {
                // Already in queue
                continue;
            }
            
            if next_height < (this_height - 1) {
                continue;
            }

            if next_height == 0 {
                // Return the first 'a' level position we reach
                return this_dist + 1;
            }

            visited.set(next_loc, this_dist + 1).unwrap();
            visit_queue.push_front(next_loc);
            
        }
    }
    
    panic!("Failed to reach a destination");
}
