#[derive(Clone, Copy, Debug)]
enum MonkeyOp {
    Add(i64),
    Mul(i64),
    Square,
}

impl MonkeyOp {
    fn eval(self, old: i64) -> i64 {
        match self {
            MonkeyOp::Add(x) => old + x,
            MonkeyOp::Mul(x) => old * x,
            MonkeyOp::Square => old * old,
        }
    }
}

fn parse_op(op_str: &str) -> MonkeyOp {
    let parts = op_str.split_whitespace().collect::<Vec<_>>();
    assert_eq!(parts[0], "old");
    
    match (parts[1], parts[2]) {
        ("*", "old") => MonkeyOp::Square,
        ("*", num_str) => MonkeyOp::Mul(num_str.parse().unwrap()),
        ("+", num_str) => MonkeyOp::Add(num_str.parse().unwrap()),
        _ => panic!("Invalid monkey operation string")
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<i64>,
    op: MonkeyOp,
    test_divisor: i64,
    true_target: usize,
    false_target: usize,
    inspection_count: i64,
}

fn parse_monkey<'a>(lines: &mut impl Iterator<Item=&'a str>) -> Monkey {
    assert!(lines.next().unwrap().starts_with("Monkey "));
    
    let items = lines.next().unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
    
    let op = lines.next().unwrap()
        .strip_prefix("  Operation: new = ")
        .map(parse_op)
        .unwrap();
    
    let test_divisor = lines.next().unwrap()
        .strip_prefix("  Test: divisible by ")
        .unwrap()
        .parse()
        .unwrap();
    
    let true_target = lines.next().unwrap()
        .strip_prefix("    If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();
    
    let false_target = lines.next().unwrap()
        .strip_prefix("    If false: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap();

    Monkey {
        items,
        op,
        test_divisor,
        true_target,
        false_target,
        inspection_count: 0,
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let new_monkey = parse_monkey(&mut lines);
        monkeys.push(new_monkey);
        
        // Skip any blank lines
        while let Some(&"") = lines.peek() {
            lines.next();
        }
    }

    monkeys
}

fn solve(input: &[Monkey], rounds: i32, reduction: impl Fn(i64) -> i64) -> i64 {
    fn round(monkeys: &mut [Monkey], reduction: impl Fn(i64) -> i64) {
        for i in 0..monkeys.len() {
            // My input has no self-referential, so safe to take + reset this first
            let mut items = Vec::new();
            std::mem::swap(&mut monkeys[i].items, &mut items);
            
            monkeys[i].inspection_count += items.len() as i64;

            for item in items {
                let new_value = monkeys[i].op.eval(item);
                let new_value = reduction(new_value);
                if new_value % monkeys[i].test_divisor == 0 {
                    monkeys[monkeys[i].true_target].items.push(new_value);
                } else {
                    monkeys[monkeys[i].false_target].items.push(new_value);
                }
            }
            
        }
    }

    let mut monkeys = input.to_vec();
    monkeys.iter_mut().for_each(|m| m.inspection_count = 0);

    for _ in 0..rounds {
        round(&mut monkeys, &reduction);
    }
    
    monkeys.sort_by_key(|m| std::cmp::Reverse(m.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

pub fn solve_part_1(input: &[Monkey]) -> i64 {
    solve(input, 20, |x| x / 3)
}

pub fn solve_part_2(input: &[Monkey]) -> i64 {
    let modulus: i64 = input.iter().map(|m| m.test_divisor).product();
    solve(input, 10_000, |x| x % modulus)
}