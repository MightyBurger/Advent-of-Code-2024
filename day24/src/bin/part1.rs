use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Net<'a> {
    // name: &'a str, // Store in HashMap instead
    value: Option<bool>,
    dependants: Option<(&'a str, &'a str)>,
    relation: Option<Relation>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Relation {
    AND,
    OR,
    XOR,
}

type Netlist<'a> = HashMap<&'a str, Net<'a>>;

fn process(input: &str) -> u64 {
    let mut netlist: Netlist = HashMap::new();
    let mut queue: Vec<&str> = Vec::new();

    let (input1, input2) = input.split_once("\n\n").unwrap();

    // Fill in the rest of the netlist.
    for line in input2.lines().filter(|line| line.len() > 0) {
        let mut tokens = line.split_whitespace();

        let in1 = tokens.next().unwrap();
        let relation = tokens.next().unwrap();
        let in2 = tokens.next().unwrap();
        let _ = tokens.next().unwrap();
        let out = tokens.next().unwrap();

        let relation = match relation {
            "AND" => Relation::AND,
            "OR" => Relation::OR,
            "XOR" => Relation::XOR,
            _ => panic!(),
        };

        let net = Net {
            value: None,
            dependants: Some((in1, in2)),
            relation: Some(relation),
        };

        netlist.insert(out, net);
    }

    // Fill in the inital values, and initialize the queue.
    for line in input1.lines().filter(|line| line.len() > 0) {
        let (netname, value) = line.split_once(": ").unwrap();
        let value = match value {
            "0" => Some(false),
            "1" => Some(true),
            _ => panic!(),
        };
        for dependant in netlist.iter().filter(|(_, net)| match net.dependants {
            Some((dep1, dep2)) if dep1 == netname || dep2 == netname => true,
            _ => false,
        }) {
            queue.push(dependant.0);
        }
        let net = Net {
            value,
            dependants: None,
            relation: None,
        };
        netlist.insert(netname, net);
    }

    while queue.len() > 0 {
        let mut next_queue: Vec<&str> = Vec::new();

        for item in queue.into_iter() {
            let net_needing_an_update = netlist.get(item).unwrap();
            let relation = net_needing_an_update
                .relation
                .expect("if we're updating something, it should have dependants");
            let (dep1, dep2) = net_needing_an_update
                .dependants
                .expect("if we're updating something, it should have dependants");
            let dep1_val = netlist.get(dep1).unwrap().value;
            let dep2_val = netlist.get(dep2).unwrap().value;
            let net_new_value: Option<bool> = match (dep1_val, dep2_val, relation) {
                (None, _, _) | (_, None, _) => None,
                (Some(false), Some(false), Relation::OR) => Some(false),
                (Some(false), Some(true), Relation::OR) => Some(true),
                (Some(true), Some(false), Relation::OR) => Some(true),
                (Some(true), Some(true), Relation::OR) => Some(true),
                (Some(false), Some(false), Relation::AND) => Some(false),
                (Some(false), Some(true), Relation::AND) => Some(false),
                (Some(true), Some(false), Relation::AND) => Some(false),
                (Some(true), Some(true), Relation::AND) => Some(true),
                (Some(false), Some(false), Relation::XOR) => Some(false),
                (Some(false), Some(true), Relation::XOR) => Some(true),
                (Some(true), Some(false), Relation::XOR) => Some(true),
                (Some(true), Some(true), Relation::XOR) => Some(false),
            };
            match net_new_value {
                None => (),
                Some(_) => {
                    for next_in_line in netlist.iter().filter(|(_, net)| match net.dependants {
                        Some((dep1, dep2)) if dep1 == item || dep2 == item => true,
                        _ => false,
                    }) {
                        next_queue.push(next_in_line.0);
                    }
                }
            }
            let net_needing_an_update = netlist.get_mut(item).unwrap();
            net_needing_an_update.value = net_new_value;
        }

        queue = next_queue;
    }

    use itertools::Itertools;
    netlist
        .iter()
        .filter(|(netname, _)| netname.chars().next().unwrap() == 'z')
        .sorted_by(|(netname1, _), (netname2, _)| {
            let num1: u32 = netname1
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();
            let num2: u32 = netname2
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();
            Ord::cmp(&num1, &num2)
        })
        .enumerate()
        .fold(0, |acc, (i, (_netname, net))| match net.value {
            Some(true) => acc + (1 << i),
            Some(false) => acc,
            None => panic!("net should have value"),
        })
}

fn main() {
    let input = include_str!("../../../day24/input1.txt");
    let result = process(input);
    println!("The result is {}", result);
}

// ----------------------------------------------------
// -------------------- Unit Tests --------------------
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short() {
        let check = include_str!("../../../day24/check1_short.txt");
        assert_eq!(process(check), 4)
    }
    #[test]
    fn test_long() {
        let check = include_str!("../../../day24/check1_long.txt");
        assert_eq!(process(check), 2024)
    }
}
