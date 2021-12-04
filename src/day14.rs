use crate::Day;
use crate::utils;

use std::collections::HashMap;
use std::str::FromStr;

pub struct Day14;

impl Day for Day14 {

    fn problem1(&self) -> i64 {
        let reactants = parse_reactions();
        let mut excess = init_excess(&reactants);
        request(&reactants, &mut excess, &"FUEL".to_string(), 1);
        *excess.get(&"ORE".to_string()).unwrap()
    }
    
    fn problem2(&self) -> i64 {
        const SUPPLY: i64 = 1000000000000;
        let reactants = parse_reactions();
        let mut excess = init_excess(&reactants);
        request(&reactants, &mut excess, &"FUEL".to_string(), 1);
        let mut lower = SUPPLY / *excess.get(&"ORE".to_string()).unwrap();
        let mut upper = lower * 2;
        let mut guess = (upper + lower) / 2;
        while guess != lower {
            let mut excess = init_excess(&reactants);
            request(&reactants, &mut excess, &"FUEL".to_string(), guess);
            let result = *excess.get(&"ORE".to_string()).unwrap();
            if result > SUPPLY {
                upper = guess;
            } else {
                lower = guess;
            }
            guess = (upper + lower) / 2;
        }
        guess
    }
}

pub fn request(reactants: &HashMap<String, Reactant>, excess: &mut HashMap<String, i64>, chemical: &String, requested: i64) {
    if chemical == "ORE" {
        *excess.get_mut(chemical).unwrap() += requested;
        return;
    }
    let mut current = *excess.get(chemical).unwrap();
    let reactant = reactants.get(chemical).unwrap();
    if requested <= current {
        current -= requested;
    } else {
        let produced = reactant.produced;
        let batches = ((requested - current) as f64 / produced as f64).ceil() as i64;
        current += (produced * batches) - requested;
        let ingredients = reactant.ingredients.clone();
        for (chemical, quantity) in ingredients {
            request(reactants, excess, &chemical, batches * quantity)
        }
    }
    excess.insert(chemical.clone(), current);
}

pub fn init_excess(reactants: &HashMap<String, Reactant>) -> HashMap<String, i64> {
    reactants
        .keys()
        .cloned()
        .map(|chemical| (chemical, 0))
        .collect::<HashMap<_, _>>()
}

pub struct Reactant {
    chemical: String,
    produced: i64,
    ingredients: Vec<(String, i64)>,
}

impl FromStr for Reactant {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input.split(" => ").collect::<Vec<_>>();
        let ingredients = parts[0]
            .split(", ")
            .map(|reactant| get_reactant(reactant))
            .collect();
        let (chemical, produced) = get_reactant(parts[1]);
        Ok(Self {
            chemical,
            produced,
            ingredients
        })
    }
}

pub fn get_reactant(reactant: &str) -> (String, i64) {
    let parts = reactant.split_whitespace().collect::<Vec<_>>();
    (parts[1].to_string(), parts[0].parse::<i64>().unwrap())
}

pub fn parse_reactions() -> HashMap<String, Reactant> {
    let mut reactants = utils::get_input_string(14)
        .split("\n")
        .map(|reaction| Reactant::from_str(reaction).unwrap())
        .map(|reactant| (reactant.chemical.clone(), reactant))
        .collect::<HashMap<_, _>>();
    reactants.insert("ORE".to_string(), Reactant {
        chemical: "ORE".to_string(),
        produced: 1,
        ingredients: Vec::new(),
    });
    reactants
}