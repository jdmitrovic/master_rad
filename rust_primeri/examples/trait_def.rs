use std::f32::consts::PI;

trait Objekat2D {
    fn povrsina(&self) -> f32;
}

struct Krug {
    r: f32,
}

impl Objekat2D for Krug {
    fn povrsina(&self) -> f32 {
        self.r * self.r * PI
    }
}
