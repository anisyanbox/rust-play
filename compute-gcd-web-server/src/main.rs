use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut nums = Vec::new();

    for arg in std::env::args().skip(1) {
        nums.push(u64::from_str(&arg)
                        .expect("error parsing argument"));
    }

    if nums.len() <= 1 {
        writeln!(std::io::stderr(), 
                 "Usage: ./compute-gcd-web-server NUMBERS")
                        .unwrap();
        std::process::exit(1);
    }

    let mut g = nums[0];
    let mut l = nums[0];
    for m in &nums[1..] {
        g = gcd(g, *m);
        l = lcm(l, *m);
    }

    println!("gcd of {:?} is {}", nums, g);
    println!("lcm of {:?} is {}", nums, l);
}

/// Calculate great common devider
fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        if b < a {
            let t = b;
            b = a;
            a = t;
        }
        b = b % a;
    }
    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(36, 48), 12);
}

/// Calculate less common multiply
fn lcm (a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    return (a * b) / (gcd(a, b));
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(14, 15), 14 * 15);
    assert_eq!(lcm(36, 48), 144);
}