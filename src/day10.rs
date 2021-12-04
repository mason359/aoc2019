use crate::Day;
use crate::utils;

use std::collections::HashMap;
use std::f32::consts::PI;

pub struct Day10;

impl Day for Day10 {

    fn problem1(&self) -> i64 {
        let input = utils::get_input_string(10);
        let map: Vec<Vec<bool>> = input
            .split_whitespace()
            .map(|row| row
                .chars()
                .map(|cell| cell == '#')
                .collect())
            .collect();
        let (_, num_asteroids) = find_optimal(&map);
        num_asteroids as i64
    }
    
    fn problem2(&self) -> i64 {
        const ASTEROID_NUM: i32 = 200;
        let input = utils::get_input_string(10);
        let map: Vec<Vec<bool>> = input
            .split_whitespace()
            .map(|row| row
                .chars()
                .map(|cell| cell == '#')
                .collect())
                .collect();
        let (origin, _) = find_optimal(&map);
        let mut asteroids = get_asteroids(&map, origin);
        let mut keys = asteroids
            .keys()
            .copied()
            .collect::<Vec<_>>();
        keys.sort_unstable();
        let mut keys = keys
            .iter()
            .cycle()
            .skip_while(|a| **a < ((1.5 * PI) * 1000.0).round() as i32);
        let mut i = 1;
        loop {
            let key = keys.next().unwrap();
            if let Some(list) = asteroids.get_mut(key) {
                if i == ASTEROID_NUM {
                    let winner = list.pop().unwrap();
                    return (winner.c * 100 + winner.r) as i64;
                }
                list.pop();
                i += 1;
                if list.is_empty() {
                    asteroids.remove(key);
                }
            }
        }
    }
}

pub fn find_optimal(map: &Vec<Vec<bool>>) -> ((i32, i32), i32) {
    let mut optimal = 0;
    let mut opt_row = 0;
    let mut opt_col = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if !map[row][col] {
                continue;
            }
            let num_asteroids = get_asteroids(&map, (row as i32, col as i32)).len() as i32;
            if num_asteroids > optimal {
                optimal = num_asteroids;
                opt_row = row;
                opt_col = col;
            }
        }
    }
    ((opt_row as i32, opt_col as i32), optimal)
}

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    r: i32,
    c: i32,
}

impl Asteroid {
    pub fn angle(&self, other: &Asteroid) -> i32 {
        ((((other.r - self.r) as f32).atan2((other.c - self.c) as f32) + (2.0 * PI)) % (2.0 * PI) * 1000.0).round() as i32
    }

    pub fn distance(&self, other: &Asteroid) -> i32 {
        (other.r - self.r).abs() + (other.c - self.c).abs()
    }
}

pub fn get_asteroids(map: &Vec<Vec<bool>>, origin: (i32, i32)) -> HashMap<i32, Vec<Asteroid>> {
    let origin = Asteroid { r: origin.0, c: origin.1 };
    let mut asteroids: HashMap<i32, Vec<Asteroid>> = HashMap::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if r as i32 == origin.r && c as i32 == origin.c {
                continue;
            }
            if map[r][c] {
                let new_asteroid = Asteroid{ r: r as i32, c: c as i32 };
                match asteroids.get_mut(&origin.angle(&new_asteroid)) {
                    Some(line) => {
                        line.push(new_asteroid);
                    }
                    None => {
                        asteroids.insert(origin.angle(&new_asteroid), vec![new_asteroid]);
                    }
                }
            }
        }
    }
    for (_, list) in &mut asteroids {
        list.sort_unstable_by_key(|a| origin.distance(a));
        list.reverse();
    }
    asteroids
}