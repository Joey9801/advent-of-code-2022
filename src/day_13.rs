use std::{cmp::{Ordering, PartialOrd, Ord}, str::FromStr};

#[derive(Clone, PartialEq, Eq)]
pub enum Packet {
    Integer(u8),
    List(Vec<Packet>)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (Some(x), Some(y)) => match x.cmp(y){
                            Ordering::Equal => continue,
                            other => break other,
                        },
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (None, None) => break Ordering::Equal,
                    }
                }
            },
            pair @ (Packet::Integer(_), Packet::List(b)) => {
                match &b[..] {
                    [] => Ordering::Greater,
                    [x] => pair.0.cmp(x),
                    [x, ..] => if pair.0 == x { Ordering::Less } else { pair.0.cmp(x) },
                }
            }
            pair @ (Packet::List(a), Packet::Integer(_)) => {
                match &a[..] {
                    [] => Ordering::Less,
                    [x] => x.cmp(&pair.1),
                    [x, ..] => if x == pair.1 { Ordering::Greater } else { x.cmp(&pair.1) },
                }
            }
        };
        Some(res)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            anyhow::bail!("Can't parse non-ascii packet")
        }

        if let Ok(i) = s.trim().parse() {
            return Ok(Self::Integer(i));
        }
        
        assert!(s.starts_with('['));
        assert!(s.ends_with(']'));
        
        let mut elements = Vec::new();
        
        let mut el_start = 1;
        let mut el_end = el_start;
        let mut depth = 0;
        loop {
            match s.chars().nth(el_end).unwrap() {
                ',' if depth == 0 => {
                    elements.push(s[el_start..el_end].parse()?);
                    el_start = el_end + 1;
                }
                '[' => depth += 1,
                ']' if depth > 0 => depth -= 1,
                ']' if depth == 0 => {
                    if el_start != el_end {
                        elements.push(s[el_start..el_end].parse()?);
                    }
                    break;
                }
                _ => (),
            }
            
            el_end += 1;
        }
        
        Ok(Packet::List(elements))
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(a) => write!(f, "{a:?}"),
            Packet::List(elements) => {
                write!(f, "[")?;
                let mut first = true;
                for el in elements {
                    if !first {
                        write!(f, ",")?;
                    }
                    first = false;
                    
                    write!(f, "{el:?}")?;
                }
                write!(f, "]")
            },
        }
    }
}


pub fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::from_str)
        .map(Result::unwrap)
        .collect()
}

pub fn solve_part_1(input: &[Packet]) -> usize {
    input
        .chunks(2)
        .enumerate()
        .filter(|(_idx, pair)| pair[0] < pair[1])
        .map(|(idx, _pair)| idx + 1)
        .sum()
}

pub fn solve_part_2(input: &[Packet]) -> usize {
    let mut all_packets = input.to_vec();
    let div_a: Packet = "[[2]]".parse().unwrap();
    let div_b: Packet = "[[6]]".parse().unwrap();

    all_packets.push(div_a.clone());
    all_packets.push(div_b.clone());
    all_packets.sort();
    
    let pos_a = all_packets.binary_search(&div_a).unwrap() + 1;
    let pos_b = all_packets.binary_search(&div_b).unwrap() + 1;
    
    pos_a * pos_b
}