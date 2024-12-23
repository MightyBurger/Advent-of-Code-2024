fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn next_secret(mut a: u64) -> u64 {
    a = prune(mix(a, a * 64));
    a = prune(mix(a, a / 32));
    a = prune(mix(a, a * 2048));
    a
}

fn secret_n(mut a: u64, n: i32) -> u64 {
    for _ in 0..n {
        a = next_secret(a)
    }
    a
}

fn process(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| secret_n(line.parse().unwrap(), 2000))
        .sum()
}

fn main() {
    let input = include_str!("../../../day22/input1.txt");
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
        let check = include_str!("../../../day22/check1.txt");
        assert_eq!(process(check), 37327623)
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_next() {
        assert_eq!(next_secret(123), 15887950);
    }

    #[test]
    fn test_each() {
        let n = 2000;
        assert_eq!(secret_n(1, n), 8685429);
        assert_eq!(secret_n(10, n), 4700978);
        assert_eq!(secret_n(100, n), 15273692);
        assert_eq!(secret_n(2024, n), 8667524);
    }
}
