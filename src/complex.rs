#[derive(Copy, Clone)]
pub struct Complex {
    pub a: f64,
    pub b: f64,
}

impl Complex {
    pub fn new(a: f64, b: f64) -> Complex {
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
