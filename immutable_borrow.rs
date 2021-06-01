fn main() {
    let s = String::from("Hello world");

    println!("Niska \"{}\" ima {} reci", s, broj_reci(&s));
}

fn broj_reci(s: &String) -> usize {
    s.matches(" ").count() + 1
}
