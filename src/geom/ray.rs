use super::vec::Vector;

pub struct Ray {
    pub or: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(or: Vector, dir: Vector) -> Self {
        Self { or: or, dir: dir }
    }

    pub fn at_param(&self, t: f64) -> Vector {
        self.or + self.dir.scale(t)
    }
}
