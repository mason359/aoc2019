use crate::Day;
use crate::utils;
use crate::intcode::Intcode;

use std::collections::HashMap;

pub struct Day13;

impl Day for Day13 {

    fn problem1(&self) -> i64 {
        let mut intcode = Intcode::new(utils::get_intcode(13));
        intcode.run();
        let out = intcode.stdout_all();
        let mut tiles = HashMap::new();
        for i in (0..out.len()).step_by(3) {
            tiles.insert((out[i], out[i + 1]), out[i + 2]);
        }
        tiles
            .iter()
            .filter(|(_, id)| **id == 2)
            .count()
            as i64
    }
    
    fn problem2(&self) -> i64 {
        let mut intcode = Intcode::new(utils::get_intcode(13));
        intcode.set(0, 2);
        let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
        let mut blocks = 1;
        let mut score = 0;
        let mut ball = 0;
        let mut paddle = 0;
        while blocks > 0 {
            intcode.run();
            let output = intcode.stdout_all();
            for i in (0..output.len()).step_by(3) {
                if output[i] == -1 {
                    score = output[i + 2];
                    continue;
                }
                match output[i + 2] {
                    3 => paddle = output[i],
                    4 => ball = output[i],
                    _ => (),
                }
                screen.insert((output[i], output[i + 1]), output[i + 2]);
            }
            blocks = screen
                .values()
                .filter(|id| **id == 2)
                .count();
            intcode.stdin((ball - paddle).signum());
        }
        score
    }
}