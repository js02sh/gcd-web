fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m != 0 {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
fn main() {
    let k = gcd(12,24);
    println!("the greates divisor of 12, 24 is {}", k);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*14*17,15*7*8),84);

}