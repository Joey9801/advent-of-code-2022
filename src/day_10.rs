#[derive(Clone, Copy, Debug)]
pub enum Instr {
    AddX(i32),
    Noop,
}

struct Cpu {
    x: i32,
    pending: Option<Instr>,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, pending: None }
    }
    
    fn step(&mut self, prog: &mut impl Iterator<Item=Instr>) {
        match self.pending {
            Some(Instr::AddX(x)) => {
                self.x += x;
                self.pending = None;
            },
            None => {
                match prog.next().unwrap() {
                    Instr::Noop => (),
                    Instr::AddX(x) => self.pending = Some(Instr::AddX(x))
                }
            },
            Some(Instr::Noop) => unreachable!(),
        }
    }
}

pub fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            match parts.next().unwrap() {
                "addx" => Instr::AddX(parts.next().unwrap().parse().unwrap()),
                "noop" => Instr::Noop,
                _ => panic!("Invalid instruction in input"),
            }
        })
        .collect()
}

pub fn solve_part_1(input: &[Instr]) -> i32 {
    let mut cpu = Cpu::new();

    let mut sum = 0;
    let mut instructions = input
        .iter()
        .cloned()
        .chain(std::iter::repeat(Instr::Noop));
    
    for i in 1..221 {
        if (i - 20) % 40 == 0 {
            sum += i as i32 * cpu.x;
        }
        cpu.step(&mut instructions);
    }

    sum
}

pub fn solve_part_2(input: &[Instr]) -> &'static str {
    let mut cpu = Cpu::new();

    let mut instructions = input
        .iter()
        .cloned()
        .chain(std::iter::repeat(Instr::Noop));
    
    let mut crt_data = Vec::new();
    for i in 1..241 {
        let x_pos = (i - 1) % 40;
        crt_data.push((cpu.x - x_pos).abs() <= 1);
        cpu.step(&mut instructions);
    }
    
    for output_line in crt_data.chunks(40) {
        let _line_string = output_line
            .iter()
            .map(|x| if *x { '#' } else { ' ' })
            .collect::<String>();
        // println!("{}", line_string);
    }

    "lolocr"
}