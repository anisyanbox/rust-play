fn main() {
    println!("Hello, rusty world!\n");
    println!("gcd = {}", gcd(36, 48));
    println!("lcm = {}", lcm(36, 48));
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