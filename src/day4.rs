use crate::Day;
use crate::utils;

pub struct Day4;

impl Day for Day4 {

    fn problem1(&self) -> i64 {
        let range = utils::get_input_string(4);
        let lower: i32 = range[..6].parse().unwrap();
        let upper: i32 = range[7..].parse().unwrap();
        (lower..=upper)
            .filter(|num| passes_rules(num))
            .count()
            as i64
    }
    
    fn problem2(&self) -> i64 {
        let range = utils::get_input_string(4);
        let lower: i32 = range[..6].parse().unwrap();
        let upper: i32 = range[7..].parse().unwrap();
        (lower..=upper)
            .filter(|num| passes_rules_strict(num))
            .count()
            as i64
    }

}

fn passes_rules(num: &i32) -> bool {
    let mut old_digit = num % 10;
    let mut new_digit;
    let mut found_double = false;
    for i in 1..6 {
        new_digit = num / 10_i32.pow(i) % 10;
        if new_digit > old_digit {
            return false;
        }
        if new_digit == old_digit {
            found_double = true;
        }
        old_digit = new_digit;
    }
    found_double
}

fn passes_rules_strict(num: &i32) -> bool {
    let mut old_digit = num % 10;
    let mut new_digit;
    let mut found_double = false;
    let mut matching = 1;
    for i in 1..6 {
        new_digit = num / 10_i32.pow(i) % 10;
        if new_digit > old_digit {
            return false;
        }
        if new_digit == old_digit {
            matching += 1;
        } else {
            if matching == 2 {
                found_double = true;
            }
            matching = 1;
        }
        old_digit = new_digit;
    }
    if matching == 2 {
        found_double = true;
    }
    found_double
}