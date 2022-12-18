/// Inclusive range of section IDs
#[derive(Clone, Copy, Debug)]
pub struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn contains(self, other: Self) -> bool {
        (self.start <= other.start) && (self.end >= other.end)
    }
    
    fn overlaps(self, other: Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
        || (self.end >= other.start && self.end <= other.end)
        || (self.start <= other.start && self.end >= other.end)
    }
}

pub fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    fn parse_assignment(x: &str) -> Assignment {
        let mut parts = x.split('-');
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        Assignment { start: std::cmp::min(a, b), end: std::cmp::max(a, b) }
    }
    
    let mut output = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(',');

        let a = parse_assignment(parts.next().unwrap());
        let b = parse_assignment(parts.next().unwrap());
        output.push((a, b));
    }
    
    output
}

pub fn solve_part_1(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.contains(*b) || b.contains(*a))
        .count()
}

pub fn solve_part_2(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter(|(a, b)| a.overlaps(*b))
        .count()
}