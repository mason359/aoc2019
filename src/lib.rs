pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub mod utils;
pub mod intcode;

pub trait Day {
    fn problem1(&self) -> i64;

    fn problem2(&self) -> i64;
}

pub fn get_day(arg: Option<String>) -> Result<i64, &'static str> {
    match arg {
        Some(day) => match day.parse::<i64>().unwrap() {
            day_num if (1..=25).contains(&day_num) => Ok(day_num),
            _ => Err("Invalid day number!"),
        }
        None => Err("No day specified!"),
    }
}

pub fn get_problem(arg: Option<String>) -> Result<i64, &'static str> {
    match arg {
        Some(problem) => match problem.parse::<i64>().unwrap() {
            day_num if day_num == 1 || day_num == 2 => Ok(day_num),
            _ => Err("Invalid problem number!"),
        }
        None => Err("No problem specified!"),
    }
}