use crate::Day;
use crate::utils;
use crate::intcode::Intcode;

use std::collections::HashMap;

pub struct Day11;

impl Day for Day11 {

    fn problem1(&self) -> i64 {
        let mut robot = Robot::new();
        robot.run(0);
        robot.get_panel_count() as i64
    }
    
    fn problem2(&self) -> i64 {
        let mut robot = Robot::new();
        robot.run(1);
        robot.display_hull();
        0
    }

}

pub struct Robot {
    intcode: Intcode,
    pos: (i32, i32),
    direction: i32,
    panels: HashMap<(i32, i32), i32>
}

impl Robot {
    pub fn new() -> Self {
        Self {
            intcode: Intcode::new(utils::get_intcode(11)),
            pos: (0, 0),
            direction: 0,
            panels: HashMap::new(),
        }
    }

    pub fn run(&mut self, mut current: i32) {
        while !self.intcode.is_halted() {
            self.intcode.stdin(current as i64);
            self.intcode.run();
            let color = self.intcode.stdout().unwrap() as i32;
            let turn = self.intcode.stdout().unwrap() as i32;
            self.panels.insert(self.pos, color);
            self.direction += if turn == 1 { 1 } else { -1 } + 4;
            self.direction %= 4;
            match self.direction {
                0 => self.pos.1 += 1,
                1 => self.pos.0 += 1,
                2 => self.pos.1 -= 1,
                3 => self.pos.0 -= 1,
                _ => ()
            }
            current = match self.panels.get(&self.pos) {
                Some(val) => *val,
                None => 0,
            };
        }
    }

    pub fn get_panel_count(&self) -> usize {
        self.panels.len()
    }

    pub fn display_hull(&self) {
        let x_min = self.panels.keys().min_by_key(|(x, _)| x).unwrap().0;
        let y_min = self.panels.keys().min_by_key(|(_, y)| y).unwrap().1;
        let x_max = self.panels.keys().max_by_key(|(x, _)| x).unwrap().0;
        let y_max = self.panels.keys().max_by_key(|(_, y)| y).unwrap().1;
        for j in (y_min..=y_max).rev() {
            for i in x_min..=x_max {
                let color = match self.panels.get(&(i, j)) {
                    Some(val) => *val,
                    None => 0,
                };
                match color {
                    0 => print!(" "),
                    1 => print!("█"),
                    _ => (),
                }
            }
            print!("\n");
        }
    }
}