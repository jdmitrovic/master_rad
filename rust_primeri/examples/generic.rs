struct Tacka<T> {
    x: T,
    y: T
}

impl Tacka<f32> {
    fn dist(&self, t: &Tacka<f32>) -> f32 {
        ((self.x - t.x).powi(2) + (self.y - t.y).powi(2)).sqrt()
    }
}

fn main() {
    let t1 = Tacka {
        x: 32.0,
        y: 40.0
    };
    let t2 = Tacka {
        x: 13.0,
        y: 51.0
    };

    println!("Rastojanje izmedju tacaka je: {}", t1.dist(&t2));
}
