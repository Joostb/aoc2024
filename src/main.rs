use clap::Parser;
use day::Day;
use std::fs::read_to_string;
mod day;
mod day1;
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
    let input = read_to_string("inputs/1.txt").unwrap();
    let result;

    let day = match args.day {
        1 => day1::Day1,
        _ => todo!(),
    };

    if args.part == 1 {
        result = Some(day.part1(&input));
    } else {
        result = Some(day.part2(&input));
    }

    result.unwrap()
}
