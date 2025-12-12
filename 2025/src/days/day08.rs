use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::{cell::RefCell, rc::Rc};
use ordered_float::OrderedFloat;

use crate::problem::Problem;

pub struct DayEight;

fn distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let re = regex::Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

        let junctions: Vec<(i32, i32, i32)> = re.captures_iter(input).map(|cap| {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            let c: i32 = cap[3].parse().unwrap();
            (a, b, c)
        }).collect();

        let mut distances: BTreeMap<OrderedFloat<f64>, (usize, usize)> = BTreeMap::new();

        for i in 0..junctions.len() {
            for j in i + 1..junctions.len() {
                let dist = distance(junctions[i], junctions[j]);
                distances.insert(OrderedFloat(dist), (i, j));
            }
        }

        let mut groups: HashMap<usize, Rc<RefCell<Vec<usize>>>> = HashMap::new();

        let testing = if junctions.len() > 200 { 1000 } else { 10 };

        for (_, (i, j)) in distances.iter().take(testing) {
            let group_i = groups.get(&i).cloned();
            let group_j = groups.get(&j).cloned();

            if group_i.is_none() && group_j.is_none() {
                // neither exist, create new
                let new_g = Rc::new(RefCell::new(vec![*i, *j]));
                groups.insert(*i, new_g.clone());
                groups.insert(*j, new_g);
            } else if group_i.is_none() {
                let g = group_j.unwrap();
                let mut g_rc = g.borrow_mut();
                g_rc.push(*i);
                groups.insert(*i, g.clone());
            } else if group_j.is_none() {
                let g = group_i.unwrap();
                let mut g_rc = g.borrow_mut();
                g_rc.push(*j);
                groups.insert(*j, g.clone());
            } else {
                // both exist, merge
                let g_i = group_i.unwrap();
                let g_j = group_j.unwrap();
                
                if Rc::ptr_eq(&g_i, &g_j) {
                    continue; // already the same group
                }

                let mut g_i_rc = g_i.borrow_mut();
                let mut g_j_rc = g_j.borrow_mut();
                g_j_rc.append(g_i_rc.as_mut());

                for member in g_j_rc.iter() {
                    groups.insert(*member, g_j.clone());
                }
            }
        }

        let mut seen: HashSet<usize> = HashSet::new();
        let mut largest: BTreeSet<usize> = BTreeSet::new();

        for (i, group) in groups.drain() {
            if seen.contains(&i) {
                continue;
            }
            let g_rc = group.borrow();
            for member in g_rc.iter() {
                seen.insert(*member);
            }
            largest.insert(g_rc.len());
        }

        let ans = largest.into_iter().rev().take(3).product::<usize>();

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {
        let re = regex::Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

        let junctions: Vec<(i32, i32, i32)> = re.captures_iter(input).map(|cap| {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            let c: i32 = cap[3].parse().unwrap();
            (a, b, c)
        }).collect();

        let mut distances: BTreeMap<OrderedFloat<f64>, (usize, usize)> = BTreeMap::new();

        for i in 0..junctions.len() {
            for j in i + 1..junctions.len() {
                let dist = distance(junctions[i], junctions[j]);
                distances.insert(OrderedFloat(dist), (i, j));
            }
        }

        let mut groups: HashMap<usize, Rc<RefCell<Vec<usize>>>> = HashMap::new();

        while let Some((_, (i, j))) = distances.pop_first() {
            let group_i = groups.get(&i).cloned();
            let group_j = groups.get(&j).cloned();

            if group_i.is_none() && group_j.is_none() {
                // neither exist, create new
                let new_g = Rc::new(RefCell::new(vec![i, j]));
                groups.insert(i, new_g.clone());
                groups.insert(j, new_g);
            } else if group_i.is_none() {
                let g = group_j.unwrap();
                let mut g_rc = g.borrow_mut();
                g_rc.push(i);
                groups.insert(i, g.clone());
            } else if group_j.is_none() {
                let g = group_i.unwrap();
                let mut g_rc = g.borrow_mut();
                g_rc.push(j);
                groups.insert(j, g.clone());
            } else {
                // both exist, merge
                let g_i = group_i.unwrap();
                let g_j = group_j.unwrap();
                
                if Rc::ptr_eq(&g_i, &g_j) {
                    continue; // already the same group
                }

                let mut g_i_rc = g_i.borrow_mut();
                let mut g_j_rc = g_j.borrow_mut();
                g_j_rc.append(g_i_rc.as_mut());

                for member in g_j_rc.iter() {
                    groups.insert(*member, g_j.clone());
                }
            }

            if let Some(group) = groups.get(&i) {
                if group.borrow().len() == junctions.len() {
                    let ans = junctions[i].0 * junctions[j].0;
                    return format!("{}", ans);
                }
            }
        }

        format!("No awnser")
    }
}
