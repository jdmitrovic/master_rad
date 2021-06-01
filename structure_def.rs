struct Student {
    ime: String,
    indeks: String,
    email: String
}

fn kreiraj_studenta(ime: String, indeks: String) -> Student {
    Student {
        ime,
        email: format!("{}@matf.bg.ac.rs", indeks),
        indeks
    }
}

fn main() {
    let student1 = kreiraj_studenta(
        String::from("Ana"),
        String::from("mi15123")
    );
    let student2 = Student {
        ime: String::from("Marko"),
        ..student1
    };
}
