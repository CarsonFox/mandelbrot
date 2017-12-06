#[derive(Copy, Clone)]
pub struct Complex {
    a: f32,
    b: f32,
}

impl Complex {
    pub fn new(a: f32, b: f32) -> Complex {
        Complex { a, b }
    }

    pub fn iterate(&mut self, c: &Complex) {
        let new_a = self.a * self.a - self.b * self.b + c.a;
        self.b = 2.0 * self.a * self.b + c.b;
        self.a = new_a;
    }

    pub fn escapes(&self) -> bool {
        self.a * self.a + self.b * self.b > 4.0
    }
}
