pub fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn solve(input: &[char], window_size: usize) -> usize {
    'outer: for (loc, window) in input.windows(window_size).enumerate() {
        for i in 0..(window_size - 1) {
            if window[(i + 1)..].iter().any(|x| *x == window[i]) {
                continue 'outer;
            }
        }

        return loc + window_size;
    }
    
    panic!("No answer")
}

pub fn solve_part_1(input: &[char]) -> usize {
    solve(input, 4)
}

pub fn solve_part_2(input: &[char]) -> usize {
    solve(input, 14)
}
