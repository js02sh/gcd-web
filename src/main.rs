use std::io;

fn gcd(mut n: u64, mut m: u64) -> u64 { // great common divisor function with Euclid's algorithm
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
    let k = gcd(12,24); //example situations
    println!("the greatest divisor of 12, 24 is {}", k);

    let mut n = String::new(); // make a terminal input
    let mut m = String::new();
    println!("Write first Number:");
    io::stdin()
        .read_line(&mut n)
        .expect("Faild to read line");
    let n: u64 = n.trim().parse()
        .expect("Please enter a number");
    println!("Write second Number:");
    io::stdin()
        .read_line(&mut m)
        .expect("Faild to read line");
    let m: u64 = m.trim().parse()
        .expect("Please enter a number");

    println!("The Greatest Common Divisor between {} and {} is {}", n, m, gcd(n,m));
    
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*14*17,15*7*8),84);

}