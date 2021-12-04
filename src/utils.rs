use std::fs;

pub fn get_input_string(day: i32) -> String {
    fs::read_to_string(format!("input/input{}.txt", day)).unwrap()
}

pub fn get_intcode(day: i32) -> Vec<i64> {
    let input = get_input_string(day);
    input
        .split_terminator(',')
        .map(|num| -> i64 { num.parse::<i64>().unwrap() })
        .collect()
}