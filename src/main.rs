use y2019::day1::Day1;
use y2019::day2::Day2;
use y2019::day3::Day3;
use y2019::day4::Day4;
use y2019::day5::Day5;
use y2019::day6::Day6;
use y2019::day7::Day7;
use y2019::day8::Day8;
use y2019::day9::Day9;
use y2019::day10::Day10;
use y2019::day11::Day11;
use y2019::day12::Day12;
use y2019::day13::Day13;
use y2019::day14::Day14;
use y2019::day15::Day15;
use y2019::day16::Day16;
use y2019::day17::Day17;
use y2019::day18::Day18;
use y2019::day19::Day19;
use y2019::day20::Day20;
use y2019::day21::Day21;
use y2019::day22::Day22;
use y2019::day23::Day23;
use y2019::day24::Day24;
use y2019::day25::Day25;

use y2019::Day;
use y2019::get_day;
use y2019::get_problem;
use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    args.next();

    let day = get_day(args.next()).unwrap();

    let problem = get_problem(args.next()).unwrap();

    let runner: Box<dyn Day> = match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
        3 => Box::new(Day3),
        4 => Box::new(Day4),
        5 => Box::new(Day5),
        6 => Box::new(Day6),
        7 => Box::new(Day7),
        8 => Box::new(Day8),
        9 => Box::new(Day9),
        10 => Box::new(Day10),
        11 => Box::new(Day11),
        12 => Box::new(Day12),
        13 => Box::new(Day13),
        14 => Box::new(Day14),
        15 => Box::new(Day15),
        16 => Box::new(Day16),
        17 => Box::new(Day17),
        18 => Box::new(Day18),
        19 => Box::new(Day19),
        20 => Box::new(Day20),
        21 => Box::new(Day21),
        22 => Box::new(Day22),
        23 => Box::new(Day23),
        24 => Box::new(Day24),
        25 => Box::new(Day25),
        _ => panic!()
    };

    let function = if problem == 1 {
        Box::new(|| -> i64 { runner.problem1() }) as Box<dyn Fn() -> _>
    } else {
        Box::new(|| -> i64 { runner.problem2() }) as Box<dyn Fn() -> _>
    };

    println!("Day {}, Problem {}:", day, problem);

    let start = Instant::now();
    let result = function();
    let time = start.elapsed().as_secs_f64();
    println!("Result: {}", result);
    println!("Finished in {}s", time);
}