fn duza_rec<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let w1 = s1.split_whitespace()
               .next()
               .unwrap_or("");

    let w2 = s2.split_whitespace()
               .next()
               .unwrap_or("");

    if w1.len() > w2.len() {
        w1
    } else {
        w2
    }
}

fn main() {
    let s1 = String::from("Prva recenica");
    let s2 = String::from("Druga recenica");

    println!("Duza rec je: {}", duza_rec(&s1, &s2));
}
