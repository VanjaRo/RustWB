// Разработать программу нахождения расстояния между двумя точками,
// которые представлены в виде структуры Point с инкапсулированными параметрами x,y и конструктором.

struct PointF64 {
    x: f64,
    y: f64,
}

impl Default for PointF64 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl PointF64 {
    fn new(x: f64, y: f64) -> PointF64 {
        PointF64 { x, y }
    }
    fn distance(&self, other: &PointF64) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }
}

fn main() {
    {
        let a = PointF64::new(1.0, 2.0);
        let b = PointF64::new(3.0, 2.0);
        assert_eq!(a.distance(&b), 2.0);
    }
    {
        let a = PointF64::new(2.0, 1.0);
        let b = PointF64::new(2.0, 5.0);
        assert_eq!(a.distance(&b), 4.0);
    }
    {
        let a = PointF64::new(2.0, 3.0);
        let b = PointF64::new(2.0, 3.0);
        assert_eq!(a.distance(&b), 0.0);
    }
}
