#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }
    pub fn area(&self) -> f64 {
        let pi: f64 = std::f64::consts::PI;
        let x = self.radius.powf(2.0);
        pi * x
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn intersect(&self, other: Circle)->bool{
        let dis = self.center.distance(other.center);
        let re = self.radius + other.radius;
       
        if dis<= re{
            return true
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let x = self.0 - other.0;
        let y = self.1 - other.1;
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }
}
