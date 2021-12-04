use crate::Day;
use crate::utils;
use crate::intcode::Intcode;

pub struct Day9;

impl Day for Day9 {

    fn problem1(&self) -> i64 {
        let program = utils::get_intcode(9);
        let mut intcode = Intcode::new(program);
        intcode.stdin(1);
        intcode.run();
        intcode.stdout().unwrap()
    }
    
    fn problem2(&self) -> i64 {
        let program = utils::get_intcode(9);
        let mut intcode = Intcode::new(program);
        intcode.stdin(2);
        intcode.run();
        intcode.stdout().unwrap()
    }

}