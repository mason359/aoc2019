use crate::Day;
use crate::utils;
use std::collections::HashMap;

const YOU: &str = "YOU";
const SAN: &str = "SAN";

pub struct Day6;

impl Day for Day6 {

    fn problem1(&self) -> i64 {
        const COM: &str = "COM";
        let input = utils::get_input_string(6);
        let orbits = get_orbits(&input[..]);
        let (_, num_orbits) = count_orbits(&orbits, COM);
        num_orbits as i64
    }
    
    fn problem2(&self) -> i64 {
        const COM: &str = "COM";
        let input = utils::get_input_string(6);
        let orbits = get_orbits(&input[..]);
        let (_, _, num_transfers) = count_transfers(&orbits, COM);
        num_transfers as i64
    }

}

fn get_orbits(input: &str) -> HashMap<&str, Vec<&str>> {
    let objects: Vec<&str> = input
        .split_whitespace()
        .collect();
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for object in objects {
        let (planet, satellite): (&str, &str) = match &object.split_terminator(")").collect::<Vec<_>>()[..] {
            [planet, satellite] => (planet, satellite),
            _ => panic!()
        };
        orbits
            .entry(planet)
            .or_insert(Vec::new())
            .push(satellite);
    }
    orbits
}

fn count_orbits(orbits: &HashMap<&str, Vec<&str>>, planet: &str) -> (i32, i32) {
    if !orbits.contains_key(planet) {
        return (1, 0);
    }
    let mut counts = (1, 0);
    for satellite in orbits.get(planet).unwrap() {
        let counts_i = count_orbits(orbits, satellite);
        counts.0 += counts_i.0;
        counts.1 += counts_i.0 + counts_i.1;
    }
    counts
}

fn count_transfers(orbits: &HashMap<&str, Vec<&str>>, planet: &str) -> (bool, bool, i32) {
    if planet == YOU {
        return (true, false, 0);
    }
    if planet == SAN {
        return (false, true, 0);
    }
    if !orbits.contains_key(planet) {
        return (false, false, 0);
    }
    let mut info = (false, false, 0);
    for satellite in orbits.get(planet).unwrap() {
        let info_i = count_transfers(orbits, satellite);
        info.0 |= info_i.0;
        info.1 |= info_i.1;
        info.2 += info_i.2;
    }
    if info.0 && info.1 {
        return info;
    }
    if info.0 || info.1 {
        info.2 += 1;
    }
    info
}