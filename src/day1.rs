use crate::Day;
use crate::utils;

pub struct Day1;

impl Day for Day1 {

    fn problem1(&self) -> i64 {
        utils::get_input_string(1)
            .split_whitespace()
            .map(|num| -> i64 { num.parse::<i64>().unwrap() / 3 - 2})
            .sum()
    }
    
    fn problem2(&self) -> i64 {
        utils::get_input_string(1)
            .split_whitespace()
            .map(|num| -> i64 { num.parse::<i64>().unwrap() })
            .map(|mut num| -> i64 {
                let mut total = 0;
                num = num / 3 - 2;
                while num > 0 {
                    total += num;
                    num = num / 3 - 2;
                }
                total
            })
            .sum()
    }
}