use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet;

fn process(input: &str) -> usize {
    let graphmap =
        UnGraphMap::<&str, ()>::from_edges(input.lines().filter_map(|line| line.split_once("-")));

    let mut interconnected = HashSet::new();

    for a in graphmap.nodes() {
        for b in graphmap.neighbors(a) {
            for c in graphmap.neighbors(b) {
                for might_be_a in graphmap.neighbors(c) {
                    if might_be_a == a {
                        let mut network = [a, b, c];
                        network.sort();
                        if network.iter().any(|str| str.chars().next() == Some('t')) {
                            interconnected.insert(network);
                        }
                    }
                }
            }
        }
    }
    dbg!(&interconnected);

    interconnected.len()
}

fn main() {
    let input = include_str!("../../../day23/input1.txt");
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
    fn test() {
        let check = include_str!("../../../day23/check1.txt");
        assert_eq!(process(check), 7)
    }
}
