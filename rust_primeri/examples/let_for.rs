fn main() {
    let mut a: u32 = 65342;
    let mut b: u32 = 23456;

    let gcd = loop {
        if b == 0 || a == b {
            break a;
        }

        if a == 0 {
            break b;
        }

        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    };

    println!("Najveci zajednicki delilac a i b je {}", gcd);
}
