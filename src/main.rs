use clap::Parser;
use day::Day;
use std::fs::read_to_string;
mod day;
mod day1;
mod day13;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: usize,
    #[arg(short, long, default_value_t = 1)]
    part: usize,
}
fn main() {
    let args = Cli::parse();
    let result = get_result(args);
    println!("{result}");
}

fn get_result(args: Cli) -> String {
    let input = read_to_string(format!("inputs/{}.txt", args.day)).unwrap();

    let day: Box<dyn Day> = match args.day {
        1 => Box::new(day1::Day1),
        2 => Box::new(day2::Day2),
        3 => Box::new(day3::Day3),
        4 => Box::new(day4::Day4),
        5 => Box::new(day5::Day5),
        6 => Box::new(day6::Day6),
        7 => Box::new(day7::Day7),
        8 => Box::new(day8::Day8),
        13 => Box::new(day13::Day13),
        17 => Box::new(day17::Day17),
        _ => todo!(),
    };

    if args.part == 1 {
        day.part1(&input).to_string()
    } else {
        day.part2(&input)
    }
}
