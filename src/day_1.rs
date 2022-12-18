pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut output = Vec::new();
    let mut working = Vec::new();

    for line in input.lines() {
        if line.len() > 0 {
            working.push(line.parse().unwrap())
        } else {
            output.push(working);
            working = Vec::new();
        }
    }
    
    if working.len() > 0 {
        output.push(working);
    }
    
    output
}

pub fn solve_part_1(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|values| values.iter().sum::<u32>())
        .max()
        .unwrap()
}

pub fn solve_part_2(input: &[Vec<u32>]) -> u32 {
    let mut totals = input
        .iter()
        .map(|values| values.iter().sum())
        .collect::<Vec<u32>>();

    totals.sort_by_key(|x| std::cmp::Reverse(*x));

    totals[..3].iter().sum()
}