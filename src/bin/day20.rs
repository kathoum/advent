use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut modules: HashMap<String, Module> =
        BufReader::new(File::open("input/day20.txt").unwrap())
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (gate, line) = if let Some(line) = line.strip_prefix('%') {
                    (Gate::FlipFlop, line)
                } else if let Some(line) = line.strip_prefix('&') {
                    (Gate::Conjunction, line)
                } else {
                    (Gate::Broadcast, line.as_str())
                };
                let (name, outputs) = line.split_once(" -> ").unwrap();
                let outputs = outputs.split(", ").map(String::from).collect();
                (
                    name.to_owned(),
                    Module {
                        gate,
                        inputs: vec![],
                        outputs,
                    },
                )
            })
            .collect();

    let inputs: HashMap<String, Vec<String>> = modules
        .keys()
        .map(|n1| {
            (
                n1.clone(),
                modules
                    .iter()
                    .filter(|(_, m2)| m2.outputs.contains(n1))
                    .map(|(n2, _)| n2.clone())
                    .collect(),
            )
        })
        .collect();
    for (n, v) in inputs {
        modules.get_mut(&n).unwrap().inputs = v;
    }
    let modules = modules;

    let mut count = [0, 0];
    let mut states: HashMap<&str, (bool, HashMap<&str, bool>)> = modules
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                (
                    false,
                    v.inputs.iter().map(|n| (n.as_str(), false)).collect(),
                ),
            )
        })
        .collect();
    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            from: "button".into(),
            to: "broadcaster".into(),
            high: false,
        });
        while let Some(pulse) = pulses.pop_front() {
            //println!("{:?}", pulse);
            count[pulse.high as usize] += 1;
            if let Some((name, module)) = modules.get_key_value(&pulse.to) {
                //println!("  {} state={:?}", name, states[name.as_str()]);
                let out = match module.gate {
                    Gate::Broadcast => Some(pulse.high),
                    Gate::FlipFlop => {
                        if !pulse.high {
                            let s = &mut states.get_mut(name.as_str()).unwrap().0;
                            *s = !*s;
                            Some(*s)
                        } else {
                            None
                        }
                    }
                    Gate::Conjunction => {
                        let s = &mut states.get_mut(name.as_str()).unwrap().1;
                        s.insert(modules.get_key_value(&pulse.from).unwrap().0, pulse.high);
                        Some(!s.values().all(|b| *b))
                    }
                };
                if let Some(out) = out {
                    //println!("  state={:?} high: {}", module, out);
                    pulses.extend(module.outputs.iter().map(|m| Pulse {
                        from: name.clone(),
                        to: m.clone(),
                        high: out,
                    }));
                }
            }
        }
    }
    println!("Day 20 Part One: {}", count[0] * count[1]);

    let mut states: HashMap<&str, (bool, HashMap<&str, bool>)> = modules
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                (
                    false,
                    v.inputs.iter().map(|n| (n.as_str(), false)).collect(),
                ),
            )
        })
        .collect();
    let mut presses = 0u64;

    let mut last_km = 0;
    let mut last_kz = 0;
    let mut last_qs = 0;
    let mut last_xj = 0;
    let mut d_km = 0;
    let mut d_kz = 0;
    let mut d_qs = 0;
    let mut d_xj = 0;

    'outer: loop {
        presses += 1;
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            from: "button".into(),
            to: "broadcaster".into(),
            high: false,
        });
        while let Some(pulse) = pulses.pop_front() {
            //println!("{:?}", pulse);
            if !pulse.high && pulse.to == "rx" {
                break 'outer;
            }
            if let Some((name, module)) = modules.get_key_value(&pulse.to) {
                //println!("  {} state={:?}", name, states[name.as_str()]);
                let out = match module.gate {
                    Gate::Broadcast => Some(pulse.high),
                    Gate::FlipFlop => {
                        if !pulse.high {
                            let s = &mut states.get_mut(name.as_str()).unwrap().0;
                            *s = !*s;
                            Some(*s)
                        } else {
                            None
                        }
                    }
                    Gate::Conjunction => {
                        let s = &mut states.get_mut(name.as_str()).unwrap().1;
                        s.insert(modules.get_key_value(&pulse.from).unwrap().0, pulse.high);
                        Some(!s.values().all(|b| *b))
                    }
                };
                if name == "gq" && pulse.high {
                    let state = &states[name.as_str()].1;
                    //print!("{} {} state={:?}  ", presses, name, state);
                    if state["km"] {
                        d_km = presses - last_km;
                        last_km = presses;
                    }
                    if state["kz"] {
                        d_kz = presses - last_kz;
                        last_kz = presses;
                    }
                    if state["qs"] {
                        d_qs = presses - last_qs;
                        last_qs = presses;
                    }
                    if state["xj"] {
                        d_xj = presses - last_xj;
                        last_xj = presses;
                    }
                    //println!("km {d_km} kz {d_kz} qs {d_qs} zj {d_xj}");
                    if d_km != 0 && d_kz != 0 && d_qs != 0 && d_xj != 0 {
                        presses = d_km * d_kz * d_qs * d_xj;
                        break 'outer;
                    }
                }
                if let Some(out) = out {
                    //println!("  state={:?} high: {}", module, out);
                    pulses.extend(module.outputs.iter().map(|m| Pulse {
                        from: name.clone(),
                        to: m.clone(),
                        high: out,
                    }));
                }
            }
        }
    }

    println!("Day 20 Part Two: {presses}");
}

#[derive(Debug)]
enum Gate {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    gate: Gate,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Pulse {
    from: String,
    to: String,
    high: bool,
}
