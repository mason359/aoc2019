use crate::Day;
use crate::utils;

pub struct Day3;

impl Day for Day3 {

    fn problem1(&self) -> i64 {
        let input = utils::get_input_string(3);
        let (wire1, wire2) = parse_input(&input);
        let intersections = get_intersections(wire1, wire2);
        intersections
            .iter()
            .filter(|i| i.x != 0 || i.y != 0)
            .min_by_key(|i| i.x.abs() + i.y.abs())
            .map(|i| i.x.abs() + i.y.abs())
            .unwrap()
            as i64
    }
    
    fn problem2(&self) -> i64 {
        let input = utils::get_input_string(3);
        let (wire1, wire2) = parse_input(&input);
        let intersections = get_intersections(wire1, wire2);
        intersections
            .iter()
            .filter(|i| i.x != 0 || i.y != 0)
            .min_by_key(|i| i.distance)
            .map(|i| i.distance)
            .unwrap()
            as i64
    }
}

pub struct Segment {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    distance: i32,
    is_horizontal: bool,
    is_negative: bool,
}

pub struct Intersection {
    x: i32,
    y: i32,
    distance: i32,
}

pub fn parse_input(input: &String) -> (Vec<&str>, Vec<&str>) {
    let wires: Vec<&str> = input
        .split_whitespace()
        .collect();
    let wire1 = wires
        .get(0)
        .unwrap()
        .split(",")
        .collect();
    let wire2 = wires
        .get(1)
        .unwrap()
        .split(",")
        .collect();
    (wire1, wire2)
}

pub fn get_segments(wire: Vec<&str>) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let mut distance = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut is_horizontal: bool;
    let mut is_negative: bool;
    for segment in wire {
        let length = segment.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
        match segment.chars().next().unwrap() {
            'L' => {
                x1 -= length;
                is_horizontal = true;
                is_negative = true;
            }
            'R' => {
                x2 += length;
                is_horizontal = true;
                is_negative = false;
            }
            'U' => {
                y2 += length;
                is_horizontal = false;
                is_negative = false;
            }
            'D' => {
                y1 -= length;
                is_horizontal = false;
                is_negative = true;
            }
            _ => panic!()
        }
        segments.push(Segment { x1, y1, x2, y2, distance, is_horizontal, is_negative });
        distance += length;
        if is_negative {
            x2 = x1;
            y2 = y1;
        } else {
            x1 = x2;
            y1 = y2;
        }
    }
    segments
}

pub fn get_intersections(wire1: Vec<&str>, wire2: Vec<&str>) -> Vec<Intersection> {
    let wire1 = get_segments(wire1);
    let wire2 = get_segments(wire2);
    let mut intersections: Vec<Intersection> = Vec::new();
    for segment1 in &wire1 {
        for segment2 in &wire2 {
            if segment1.is_horizontal == segment2.is_horizontal {
                continue;
            }
            let h = if segment1.is_horizontal { segment1 } else { segment2 };
            let v = if segment1.is_horizontal { segment2 } else { segment1 };
            if (h.x1..=h.x2).contains(&v.x1) && (v.y1..=v.y2).contains(&h.y1) {
                let distance1 = h.distance + if h.is_negative { h.x2 - v.x1 } else { v.x1 - h.x1 };
                let distance2 = v.distance + if v.is_negative { v.y2 - h.y1 } else { h.y1 - v.y1 };
                intersections.push(Intersection {
                    x: v.x1,
                    y: h.y1,
                    distance: distance1 + distance2,
                });
            }
        }
    }
    intersections
}