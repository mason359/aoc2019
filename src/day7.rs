use crate::Day;
use crate::utils;
use crate::intcode::Intcode;
use itertools::Itertools;

pub struct Day7;

impl Day for Day7 {

    fn problem1(&self) -> i64 {
        let program = utils::get_intcode(7);
        (0..5)
            .permutations(5)
            .map(|setting| test_amplifiers(&program, setting))
            .max()
            .unwrap()
    }
    
    fn problem2(&self) -> i64 {
        let program = utils::get_intcode(7);
        (5..10)
            .permutations(5)
            .map(|setting| test_amplifiers_loop(&program, setting))
            .max()
            .unwrap()
    }

}

fn test_amplifiers(program: &Vec<i64>, setting: Vec<i64>) -> i64 {
    let mut intcodes = Vec::new();
    for i in 0..5 {
        intcodes.push(Intcode::new(program.clone()));
        intcodes[i].stdin(setting[i]);
    }
    let mut input = 0;
    for i in 0..5 {
        intcodes[i].stdin(input);
        intcodes[i].run();
        input = intcodes[i].stdout().unwrap();
    }
    input
}

fn test_amplifiers_loop(program: &Vec<i64>, setting: Vec<i64>) -> i64 {
    let mut intcodes = Vec::new();
    for i in 0..5 {
        intcodes.push(Intcode::new(program.clone()));
        intcodes[i].stdin(setting[i]);
    }
    let mut input = 0;
    loop {
        for i in 0..5 {
            intcodes[i].stdin(input);
            intcodes[i].run();
            input = intcodes[i].stdout().unwrap();
        }
        if intcodes[0].is_halted() {
            break;
        }
    }
    input
}