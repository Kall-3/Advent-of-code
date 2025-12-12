use crate::problem::Problem;

pub struct DayNineteen;

use std::collections::HashMap;
use regex::Regex;

struct WorkflowRule {
    target: usize,
    op: Box<dyn Fn(u32) -> bool>,
    op_type: String,
    breakpoint: u32,
    dest: String,
}

impl WorkflowRule {
    fn from_parts(target: &str, op: &str, num: &str, dest: &str) -> Self {
        let target = match target {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => panic!("Invalid target"),
        };

        let num = num.parse::<u32>().unwrap();

        let breakpoint = num;

        let op_type = op.to_string();

        let op = match op {
            ">" => Box::new(move |v: u32| v > num) as Box<dyn Fn(u32) -> bool>,
            "<" => Box::new(move |v: u32| v < num) as Box<dyn Fn(u32) -> bool>,
            "T" => Box::new(|_| true) as Box<dyn Fn(u32) -> bool>,
            _ => panic!("Invalid operator"),
        };

        let dest = dest.to_string();

        Self { target, op, op_type, breakpoint, dest }
    }
}

impl Problem for DayNineteen {
    fn part_one(&self, input: &str) -> String {

        let (workflows_list, parts_list) = input.split_once("\n\n").unwrap();

        let line_re = Regex::new(r"(?P<name>[a-z]+)\{(?P<body>[^}]+)\}").unwrap();
        let rule_re = Regex::new(r"(?P<target>[xmas])(?P<op>[<>])(?P<num>(\d+)):(?P<dest>[a-zA-Z]+)").unwrap();

        let mut workflows: HashMap<String, Vec<WorkflowRule>> = HashMap::new();

        for line in line_re.captures_iter(workflows_list) {
            let name = &line["name"];
            let body = &line["body"];

            for rule in body.split(',') {
                if let Some(cap) = rule_re.captures(rule) {
                    let target = &cap["target"];
                    let op = &cap["op"];
                    let num = &cap["num"];
                    let dest = &cap["dest"];

                    let workflow_rule = WorkflowRule::from_parts(target, op, num, dest);

                    workflows.entry(name.to_string())
                        .or_default()
                        .push(workflow_rule);
                } else {
                    // default rule
                    let workflow_rule = WorkflowRule::from_parts("x", "T", "0", rule);

                    workflows.entry(name.to_string())
                        .or_default()
                        .push(workflow_rule);
                }
            }
        }

        let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

        let mut ans: usize = 0;

        for cap in part_re.captures_iter(parts_list) {
            let x: u32 = cap[1].parse().unwrap();
            let m: u32 = cap[2].parse().unwrap();
            let a: u32 = cap[3].parse().unwrap();
            let s: u32 = cap[4].parse().unwrap();

            let mut current_workflow = "in".to_string();

            loop {
                let rules = workflows.get(&current_workflow).unwrap();

                for rule in rules {
                    let value = match rule.target {
                        0 => x,
                        1 => m,
                        2 => a,
                        3 => s,
                        _ => panic!("Invalid target"),
                    };

                    if (rule.op)(value) {
                        current_workflow = rule.dest.clone();
                        break;
                    }
                }

                if current_workflow == "A" {
                    ans += (x + m + a + s) as usize;
                    break;
                }

                if current_workflow == "R" {
                    break;
                }
            }
        }

        format!("{}", ans)
    }

    fn part_two(&self, input: &str) -> String {

        let (workflows_list, _) = input.split_once("\n\n").unwrap();

        let line_re = Regex::new(r"(?P<name>[a-z]+)\{(?P<body>[^}]+)\}").unwrap();
        let rule_re = Regex::new(r"(?P<target>[xmas])(?P<op>[<>])(?P<num>(\d+)):(?P<dest>[a-zA-Z]+)").unwrap();

        let mut workflows: HashMap<String, Vec<WorkflowRule>> = HashMap::new();

        for line in line_re.captures_iter(workflows_list) {
            let name = &line["name"];
            let body = &line["body"];

            for rule in body.split(',') {
                if let Some(cap) = rule_re.captures(rule) {
                    let target = &cap["target"];
                    let op = &cap["op"];
                    let num = &cap["num"];
                    let dest = &cap["dest"];

                    let workflow_rule = WorkflowRule::from_parts(target, op, num, dest);

                    workflows.entry(name.to_string())
                        .or_default()
                        .push(workflow_rule);
                } else {
                    // default rule
                    let workflow_rule = WorkflowRule::from_parts("x", "T", "0", rule);

                    workflows.entry(name.to_string())
                        .or_default()
                        .push(workflow_rule);
                }
            }
        }

        let mut ans: usize = 0;

        // consider ranges from 1 to 4000
        let mut queue: Vec<(String, u32, (u32, u32), (u32, u32), (u32, u32), (u32, u32))> = Vec::new();
        queue.push(("in".to_string(), 0, (1, 4000), (1, 4000), (1, 4000), (1, 4000)));

        while let Some((current_workflow, current_rule, x_range, m_range, a_range, s_range)) = queue.pop() {
            if current_workflow == "A" {
                let x_count = (x_range.1 - x_range.0 + 1) as usize;
                let m_count = (m_range.1 - m_range.0 + 1) as usize;
                let a_count = (a_range.1 - a_range.0 + 1) as usize;
                let s_count = (s_range.1 - s_range.0 + 1) as usize;

                ans += x_count * m_count * a_count * s_count;
                continue;
            }

            if current_workflow == "R" {
                continue;
            }

            let rules = workflows.get(&current_workflow).unwrap();
            let rule = &rules[current_rule as usize];

            let target_range = match rule.target {
                0 => x_range,
                1 => m_range,
                2 => a_range,
                3 => s_range,
                _ => panic!("Invalid target"),
            };

            let (passed_range, failed_range) = if rule.op_type == ">" {
                // greater than
                ((rule.breakpoint + 1, target_range.1), (target_range.0, rule.breakpoint))
            } else if rule.op_type == "<" {
                // less than
                ((target_range.0, rule.breakpoint - 1), (rule.breakpoint, target_range.1))
            } else {
                // always true
                (target_range, (1, 0)) // second range is invalid
            };

            // enqueue first range
            let (new_x_range, new_m_range, new_a_range, new_s_range) = match rule.target {
                0 => (passed_range, m_range, a_range, s_range),
                1 => (x_range, passed_range, a_range, s_range),
                2 => (x_range, m_range, passed_range, s_range),
                3 => (x_range, m_range, a_range, passed_range),
                _ => panic!("Invalid target"),
            };

            queue.push((rule.dest.clone(), 0, new_x_range, new_m_range, new_a_range, new_s_range));

            // enqueue second range
            if failed_range.0 <= failed_range.1 {
                let (new_x_range, new_m_range, new_a_range, new_s_range) = match rule.target {
                    0 => (failed_range, m_range, a_range, s_range),
                    1 => (x_range, failed_range, a_range, s_range),
                    2 => (x_range, m_range, failed_range, s_range),
                    3 => (x_range, m_range, a_range, failed_range),
                    _ => panic!("Invalid target"),
                };

                queue.push((current_workflow.clone(), current_rule + 1, new_x_range, new_m_range, new_a_range, new_s_range));
            }
        }

        format!("{}", ans)
    }
}
