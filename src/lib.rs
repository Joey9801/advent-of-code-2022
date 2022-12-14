use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct DayName {
    pub name: &'static str,
    pub day: u8,
}

#[derive(Debug)]
pub struct RunResult {
    pub name: DayName,
    pub parse_time: Duration,
    pub p1_time: Duration,
    pub p2_time: Duration,
    pub p1_result: String,
    pub p2_result: String,
}

impl RunResult {
    pub fn total_time(&self) -> Duration {
        self.parse_time + self.p1_time + self.p2_time
    }
}

pub struct Day<ParsedInput, P1Input, P1Result, P2Input, P2Result>
where
    ParsedInput: AsRef<P1Input> + AsRef<P2Input>,
    P1Input: ?Sized,
    P1Result: std::fmt::Display,
    P2Input: ?Sized,
    P2Result: std::fmt::Display,
{
    name: DayName,
    parse: Box<dyn Fn(&str) -> ParsedInput>,
    part_1: Box<dyn Fn(&P1Input) -> P1Result>,
    part_2: Box<dyn Fn(&P2Input) -> P2Result>,
}

pub trait ErasedDay {
    fn name(&self) -> DayName;
    fn run(&self, input: &str) -> RunResult;
}

impl<ParsedInput, P1Input, P1Result, P2Input, P2Result> ErasedDay
    for Day<ParsedInput, P1Input, P1Result, P2Input, P2Result>
where
    ParsedInput: AsRef<P1Input> + AsRef<P2Input>,
    P1Input: ?Sized,
    P1Result: std::fmt::Display,
    P2Input: ?Sized,
    P2Result: std::fmt::Display,
{
    fn name(&self) -> DayName {
        self.name
    }

    fn run(&self, input: &str) -> RunResult {
        let sw = Instant::now();
        let parsed_input = (self.parse)(input);
        let parse_time = sw.elapsed();

        let sw = Instant::now();
        let p1_result = (self.part_1)(parsed_input.as_ref());
        let p1_time = sw.elapsed();
        let p1_result = format!("{}", p1_result);

        let sw = Instant::now();
        let p2_result = (self.part_2)(parsed_input.as_ref());
        let p2_time = sw.elapsed();
        let p2_result = format!("{}", p2_result);

        RunResult {
            name: self.name.clone(),
            parse_time,
            p1_time,
            p2_time,
            p1_result,
            p2_result,
        }
    }
}

fn make_day<ParsedInput, P1Input, P1Result, P2Input, P2Result>(
    name: DayName,
    parse: impl Fn(&str) -> ParsedInput + 'static,
    part_1: impl Fn(&P1Input) -> P1Result + 'static,
    part_2: impl Fn(&P2Input) -> P2Result + 'static,
) -> Box<dyn ErasedDay>
where
    ParsedInput: AsRef<P1Input> + AsRef<P2Input> + 'static,
    P1Input: ?Sized + 'static,
    P1Result: std::fmt::Display + 'static,
    P2Input: ?Sized + 'static,
    P2Result: std::fmt::Display + 'static,
{
    Box::new(Day {
        name,
        parse: Box::new(parse),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
    })
}

macro_rules! define_days {
    (($($name:literal, $day_num:literal, $mod:ident),*)) => {
        $(
            mod $mod;
        )*

        pub fn all_days() -> Vec<Box<dyn ErasedDay>> {
            vec![$(
                make_day(
                    DayName { name: $name, day: $day_num },
                    $mod::parse,
                    $mod::solve_part_1,
                    $mod::solve_part_2,
                )
            ),*]
        }
    }
}

define_days!{
    ("Example day", 0, example_day)
}

pub fn get_input(input_root: &std::path::Path, day_name: DayName) -> Result<String, std::io::Error> {
    let file_name = format!("input_{}.txt", day_name.day);
    let mut path = input_root.to_path_buf();
    path.push(file_name);
    dbg!(&path);
    std::fs::read_to_string(path)
}