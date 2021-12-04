use crate::Day;
use crate::intcode::Intcode;
use crate::utils;

pub struct Day2;

impl Day for Day2 {

    fn problem1(&self) -> i64 {
        let mut intcode = Intcode::new(utils::get_intcode(2));
        intcode.set(1, 12);
        intcode.set(2, 2);
        intcode.run();
        intcode.get(0)
    }
    
    fn problem2(&self) -> i64 {
        const TARGET: i64 = 19690720;
        let program = utils::get_intcode(2);
        let mut intcode;
        for noun in 0..=99 {
            for verb in 0..=99 {
                intcode = Intcode::new(program.clone());
                intcode.set(1, noun);
                intcode.set(2, verb);
                intcode.run();
                if intcode.get(0) == TARGET {
                    return 100 * noun + verb;
                }
            }
        }
        -1
    }

}