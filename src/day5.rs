use crate::Day;
use crate::intcode::Intcode;
use crate::utils;

pub struct Day5;

impl Day for Day5 {

    fn problem1(&self) -> i64 {
        let mut intcode = Intcode::new(utils::get_intcode(5));
        intcode.stdin(1);
        intcode.run();
        match intcode.stdout_all().pop() {
            Some(code) => code,
            None => panic!("No output received!")
        }
    }
    
    fn problem2(&self) -> i64 {
        let mut intcode = Intcode::new(utils::get_intcode(5));
        intcode.stdin(5);
        intcode.run();
        match intcode.stdout() {
            Some(code) => code,
            None => panic!("No output received!")
        }
    }

}