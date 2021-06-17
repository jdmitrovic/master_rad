fn main() {
    let a: Option<i8> = Some(10);

    match a {
        Some(x) => println!("Vrednost promenljive a je: {}", x),
        None => println!("Promenljiva a nema vrednost!"),
    }
}
