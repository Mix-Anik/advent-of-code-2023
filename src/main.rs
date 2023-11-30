struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn perimeter(&self) -> f64 {
        return self.a + self.b + self.c
    }
}

fn main() {
    let angle: f64 = 36.0;
    let res = angle.sin();
    let tri: Triangle = Triangle{a: 2.0, b: 4.5, c: 8.0};
    println!("Sin: {}", res);
    println!("Perimeter: {}", tri.perimeter());
}
