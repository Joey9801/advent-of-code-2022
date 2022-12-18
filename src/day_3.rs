pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    
    fn value(x: u8) -> u8 {
        match x {
            b'a'..=b'z' => x - b'a' + 1,
            b'A'..=b'Z' => x - b'A' + 27,
            _ => panic!("Invalid char in input"),
        }
    }

    for line in input.lines() {
        output.push(line.bytes().map(value).collect());
    }

    output
}

pub fn solve_part_1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|items| {
            let len = items.len() / 2;
            
            // This actually runs like 10x faster than hashsets on my input/machine
            // no heap allocations + cache locality is one helluva drug
            items[..len]
                .iter()
                .filter(|x| items[len..].contains(*x))
                .map(|x| *x as u32)
                .next()
                .unwrap()
        })
        .sum()
}

pub fn solve_part_2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|chunk| {
            chunk[0].iter()
                .filter(|a| chunk[1..].iter().all(|b| b.contains(*a)))
                .next()
                .unwrap()
        })
        .map(|x| *x as u32)
        .sum()
}
