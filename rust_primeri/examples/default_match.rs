enum Boja {
    RGB(u32, u32, u32),
    CYMK(u32, u32, u32, u32),
    HSV(u32, u32, u32),
}

fn main() {
    let b = Boja::RGB(15, 233, 121);

    match b {
        Boja::RGB(r, g, b) => {
            println!("Crvena: {}, Zelena: {}, Plava: {}", &r, &g, &b);
        }
        _ => println!("Nije uneta boja u RGB modelu!"),
    }
}
