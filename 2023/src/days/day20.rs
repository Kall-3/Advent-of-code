use std::collections::HashMap;

use crate::problem::Problem;

pub struct DayTwenty;

use regex::Regex;

trait Module {
    fn process(&mut self, sender: &str, signal: bool, queue: &mut Vec<(String, bool)>);
    fn transmit(&mut self, signal: bool, queue: &mut Vec<(String, String, bool)>);
    fn notify(&mut self, sender: &str);
}

struct FlipFlop {
    name: String,
    receivers: Vec<String>,
    state: bool,
}

impl FlipFlop {
    fn new(name: String, receivers: Vec<String>) -> Self {
        Self { name, receivers, state: false }
    }
}

impl Module for FlipFlop {
    fn process(&mut self, _sender: &str, signal: bool, transmit_queue: &mut Vec<(String, bool)>) {
        if !signal {
            if !self.state {
                self.state = true;
                transmit_queue.push((self.name.clone(), true));
            } else {
                self.state = false;
                transmit_queue.push((self.name.clone(), false));
            }
        }
    }
    
    fn transmit(&mut self, signal: bool, broadcast_queue: &mut Vec<(String, String, bool)>) {
        for receiver in &self.receivers {
            println!("{} -{}-> {}", self.name, if !signal { "low" } else { "high" }, receiver);
            broadcast_queue.push((self.name.clone(), receiver.clone(), signal));
        }
    }

    fn notify(&mut self, _sender: &str) {}
}

struct Conjunction {
    name: String,
    receivers: Vec<String>,
    sender_signals: HashMap<String, bool>,
}

impl Conjunction {
    fn new(name: String, receivers: Vec<String>) -> Self {
        Self { name, receivers, sender_signals: HashMap::new() }
    }
}

impl Module for Conjunction {
    fn process(&mut self, sender: &str, signal: bool, transmit_queue: &mut Vec<(String, bool)>) {
        self.sender_signals.insert(sender.to_string(), signal);

        if self.sender_signals.values().all(|&s| s) {
            transmit_queue.push((self.name.clone(), false));
        } else {
            transmit_queue.push((self.name.clone(), true));
        }
    }

    fn transmit(&mut self, signal: bool, broadcast_queue: &mut Vec<(String, String, bool)>) {
        for receiver in &self.receivers {
            println!("{} -{}-> {}", self.name, if !signal { "low" } else { "high" }, receiver);
            broadcast_queue.push((self.name.clone(), receiver.clone(), signal));
        }
    }

    fn notify(&mut self, sender: &str) {
        self.sender_signals.insert(sender.to_string(), false);
    }
}

struct Broadcaster {
    name: String,
    receivers: Vec<String>,
}

impl Broadcaster {
    fn new(receivers: Vec<String>) -> Self {
        Self { name: "broadcaster".to_string(), receivers }
    }
}

impl Module for Broadcaster {
    fn process(&mut self, _sender: &str, signal: bool, transmit_queue: &mut Vec<(String, bool)>) {
        transmit_queue.push((self.name.clone(), signal));
    }

    fn transmit(&mut self, signal: bool, broadcast_queue: &mut Vec<(String, String, bool)>) {
        for receiver in &self.receivers {
            println!("{} -{}-> {}", self.name, if !signal { "low" } else { "high" }, receiver);
            broadcast_queue.push((self.name.clone(), receiver.clone(), signal));
        }
    }

    fn notify(&mut self, _sender: &str) { }
}

fn send_signal(modules: &mut HashMap<String, Box<dyn Module>>, button_presses: usize) -> usize {
    let mut broadcast_queue: Vec<(String, String, bool)> = Vec::new();
    let mut transmit_queue: Vec<(String, bool)> = Vec::new();

    let mut low_signal_count = 0;
    let mut high_signal_count = 0;

    for _ in 0..button_presses {
        broadcast_queue.push(("button".to_string(), "broadcaster".to_string(), false));

        while !broadcast_queue.is_empty() {
            while let Some((sender_name, receiver_name, signal)) = broadcast_queue.pop() {
                if let Some(module) = modules.get_mut(&receiver_name) {
                    module.process(&sender_name, signal, &mut transmit_queue);
                }
            }

            low_signal_count += transmit_queue.iter().filter(|&(_, signal)| !*signal).count();
            high_signal_count += transmit_queue.iter().filter(|&(_, signal)| *signal).count();

            while let Some((sender_name, signal)) = transmit_queue.pop() {
                if let Some(module) = modules.get_mut(&sender_name) {
                    module.transmit(signal, &mut broadcast_queue);
                }
            }
        }
    }

    low_signal_count * high_signal_count
}



impl Problem for DayTwenty {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"(?P<mod>[%&]?)(?P<sender>\w+) -> (?P<receivers>[\w, ]+)").unwrap();

        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();

        for cap in re.captures_iter(input) {
            let module = &cap["mod"];
            let sender = &cap["sender"].to_string();
            let receivers = &cap["receivers"].split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let module_box: Box<dyn Module> = match module {
                "%" => Box::new(FlipFlop::new(sender.clone(), receivers.clone())),
                "&" => { Box::new(Conjunction::new(sender.clone(), receivers.clone())) },
                _ => Box::new(Broadcaster::new(receivers.clone())),
            };

            connections.insert(sender.clone(), receivers.clone());
            modules.insert(sender.clone(), module_box);
        }

        // add sender names to conjunction modules
        for (sender, receivers) in connections.iter_mut() {
            for receiver in receivers {
                if let Some(module) = modules.get_mut(receiver) {
                    module.notify(sender);
                }
            }
        }

        format!("{}", send_signal(&mut modules, 1))
    }

    fn part_two(&self, _input: &str) -> String {
        format!("Not implemented")
    }
}
