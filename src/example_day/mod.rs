pub fn parse(_input: &str) -> Vec<u32> {
    vec![1, 2, 3]
}

pub fn solve_part_1(input: &[u32]) -> String {
    format!("{:?}", input)
}

pub fn solve_part_2(input: &[u32]) -> u32 {
    input.iter().sum()
}