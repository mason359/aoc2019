use crate::Day;
use crate::utils;

use std::collections::HashSet;
use std::str::FromStr;
use num::integer::lcm;

pub struct Day12;

impl Day for Day12 {

    fn problem1(&self) -> i64 {
        const STEPS: i32 = 1000;
        let mut moons = parse_input();
        for _ in 0..STEPS {
            moons = update_moons(moons);
        }
        moons
            .iter()
            .map(|moon| moon.get_energy())
            .sum::<i32>()
            as i64
    }
    
    fn problem2(&self) -> i64 {
        let mut moons = parse_input();
        let mut axis_states: Vec<HashSet<String>> = (0..3).map(|_| HashSet::new()).collect();
        let mut axis_cycles = vec![0, 0, 0];
        let mut steps = 0;
        while axis_cycles.iter().any(|&i| i == 0) {
            for i in 0..3 {
                if !axis_states[i].insert(get_axis(&moons, i)) && axis_cycles[i] == 0 {
                    axis_cycles[i] = steps;
                }
            }
            steps += 1;
            moons = update_moons(moons);
        }
        lcm(axis_cycles[0], lcm(axis_cycles[1], axis_cycles[2]))
    }
}

pub fn parse_input() -> Vec<Moon> {
    let input = utils::get_input_string(12);
    input
        .split('\n')
        .map(|coords| Moon::from_str(coords).unwrap())
        .collect()
}

pub fn update_moons(mut moons: Vec<Moon>) -> Vec<Moon> {
    for i in 0..moons.len() {
        let mut dv = vec![0, 0, 0];
        for moon2 in &moons {
            dv = dv.iter().zip(moons[i].compare_vel(moon2).iter()).map(|(total, m)| total + m).collect::<Vec<_>>();
        }
        moons[i].update_vel(dv);
    }
    for moon in &mut moons {
        moon.update_pos();
    }
    moons
}

pub fn get_axis(moons: &Vec<Moon>, i: usize) -> String {
    let mut state = Vec::new();
    for moon in moons {
        state.push(moon.pos[i]);
        state.push(moon.vel[i]);
    }
    format!("{:?}", state)
}

#[derive(Debug)]
pub struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    pub fn compare_vel(&self, other: &Moon) -> [i32; 3] {
        let mut dv = [0; 3];
        for i in 0..3 {
            dv[i] += (other.pos[i] - self.pos[i]).signum();
        }
        dv
    }

    pub fn update_vel(&mut self, dv: Vec<i32>) {
        for i in 0..3 {
            self.vel[i] += dv[i];
        }
    }

    pub fn update_pos(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    pub fn get_energy(&self) -> i32 {
        self.pos.iter().map(|i| i.abs()).sum::<i32>() * self.vel.iter().map(|i| i.abs()).sum::<i32>()
    }
}

impl FromStr for Moon {
    type Err = std::num::ParseIntError;

    fn from_str(coords: &str) -> Result<Self, Self::Err> {
        let x = coords.chars().skip(3).take_while(|c| *c != ',').collect::<String>().parse::<i32>()?;
        let y = coords.chars().skip_while(|c| *c != 'y').skip(2).take_while(|c| *c != ',').collect::<String>().parse::<i32>()?;
        let z = coords.chars().skip_while(|c| *c != 'z').skip(2).take_while(|c| *c != '>').collect::<String>().parse::<i32>()?;
        Ok(Self { pos: [x, y, z], vel: [0, 0, 0] })
    }
}