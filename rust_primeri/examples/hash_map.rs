fn main() {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    m.insert(String::from("Pas"), 2);
    m.insert(String::from("Macka"), 3);

    let p = m.entry(String::from("Pas")).or_insert(4);
    *p += 5;

    println!("{:?}", m);
}
