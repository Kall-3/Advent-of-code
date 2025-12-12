use std::collections::BinaryHeap;
use regex::Regex;
use z3::{Optimize, SatResult, ast::Int};

use crate::problem::Problem;

#[derive(Clone)]
struct IndicatorWiring {
    lights: Vec<bool>,
    wires: Vec<usize>,
}

impl IndicatorWiring {
    fn new(lights: Vec<bool>, wires: Vec<usize>) -> Self {
        Self {
            lights,
            wires,
        }
    }
}

impl PartialEq for IndicatorWiring {
    fn eq(&self, other: &Self) -> bool {
        self.lights.eq(&other.lights) && self.wires.eq(&other.wires)
    }
}

impl Eq for IndicatorWiring {}

impl PartialOrd for IndicatorWiring {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IndicatorWiring {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let matching_lights = &self.wires.iter()
            .map(|wire| self.lights[*wire])
            .filter(|&light| light)
            .count();
        let other_matching_lights = &other.wires.iter()
            .map(|wire| other.lights[*wire])
            .filter(|&light| light)
            .count();
        matching_lights.cmp(&other_matching_lights)
            .then(other.wires.len().cmp(&self.wires.len()))
            .then(other.wires.get(0).cmp(&self.wires.get(0)))
    }
}

impl std::fmt::Debug for IndicatorWiring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wires: {:?})", self.wires)
    }
}

fn try_wirings(
    light_diagram: &Vec<bool>,
    current_lights: &mut Vec<bool>,
    button_wirings: &mut BinaryHeap<IndicatorWiring>,
    current_pressed: u32,
    best_solution: &mut u32,
) {
    if current_pressed >= *best_solution {
        return;
    }

    if current_lights.iter().zip(light_diagram.iter()).all(|(&a, &b)| a == b) {
        if current_pressed < *best_solution {
            *best_solution = current_pressed;
        }
        return;
    }

    let wiring = match button_wirings.pop() {
        Some(w) => w,
        None => return,
    };

    for &wire in &wiring.wires {
        current_lights[wire] = !current_lights[wire];
    }

    try_wirings(
        light_diagram,
        current_lights,
        button_wirings,
        current_pressed + 1,
        best_solution,
    );

    for &wire in &wiring.wires {
        current_lights[wire] = !current_lights[wire];
    }

    try_wirings(
        light_diagram,
        current_lights,
        button_wirings,
        current_pressed,
        best_solution,
    );

    button_wirings.push(wiring);
}

pub struct DayTen;

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {

        let light_diagram_re = Regex::new(r"\[([.#]+)\]").unwrap();
        let button_wiring_re = Regex::new(r"\(([\d,]+)\)").unwrap();

        let mut total_pressed: u32 = 0;

        for line in input.lines() {
            let light_diagram: Vec<bool> = light_diagram_re.captures(line).unwrap()
                .get(1).unwrap().as_str()
                .chars()
                .map(|c| c == '#')
                .collect();

            let mut button_wirings: BinaryHeap<IndicatorWiring> = BinaryHeap::new();

            for cap in button_wiring_re.captures_iter(line) {
                let wiring: Vec<usize> = cap.get(1).unwrap().as_str()
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let wiring = IndicatorWiring::new(light_diagram.clone(), wiring);
                button_wirings.push(wiring);
            }

            let mut current_lights: Vec<bool> = vec![false; light_diagram.len()];
            let mut best_solution = u32::MAX;

            try_wirings(
                &light_diagram,
                &mut current_lights,
                &mut button_wirings,
                0,
                &mut best_solution,
            );

            total_pressed += best_solution;
        }

        format!("{}", total_pressed)
    }
    // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    fn part_two(&self, input: &str) -> String {
        let button_wiring_re = Regex::new(r"\(([\d,]+)\)").unwrap();
        let total_joltage_re = Regex::new(r"\{([\d,]+)\}").unwrap();

        let mut total_pressed: u32 = 0;

        for line in input.lines() {
            let joltage: Vec<i32> = total_joltage_re.captures(line).unwrap()
                .get(1).unwrap().as_str()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            let wiring_len = button_wiring_re.captures_iter(line).count();
            let mut buttons: Vec<Vec<i32>> = vec![vec![0; joltage.len()]; wiring_len];

            for (i, cap) in button_wiring_re.captures_iter(line).enumerate() {
                let wiring_indexes: Vec<usize> = cap.get(1).unwrap().as_str()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();

                for j in wiring_indexes.iter() {
                    buttons[i][*j] = 1;
                }
            }

            let ans = z3_minimization(buttons, joltage);
            total_pressed += ans as u32;
        }

        format!("{}", total_pressed)
    }
}

fn z3_minimization(buttons: Vec<Vec<i32>>, joltage: Vec<i32>) -> usize {
    let opt = Optimize::new();

    // initialize variables
    let vars: Vec<Int> = (0..buttons.len())
        .map(|i| Int::fresh_const(&format!("button_{}", i)))
        .collect();

    // assert non negative presses
    for var in &vars {
        opt.assert(&var.ge(0));
    }

    // add constraints
    for (i, &t_i) in joltage.iter().enumerate() {
        let mut sum = Int::from_i64(0);

        for (j, btn) in buttons.iter().enumerate() {
            let coeff = btn[i];
            if coeff == 0 {
                continue;
            }
            sum += &vars[j] * coeff;
        }

        opt.assert(&sum.eq(t_i));
    }

    let total_presses = vars.iter()
        .fold(Int::from_i64(0), |acc, var| acc + var);
    opt.minimize(&total_presses);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let mut total: usize = 0;
            for var in &vars {
                let val = model.eval(var, true).unwrap().as_i64().expect("Failed to get integer value");
                total += val as usize;
            }
            total
        }
        _ => panic!("No solution found"),
    }
}

