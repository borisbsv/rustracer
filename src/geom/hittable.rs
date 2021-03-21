use super::ray::Ray;
use super::vec::Vector;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (f64, Vector, Vector);
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (f64, Vector, Vector) {
        let mut closest = t_max;
        let mut t: f64 = 0.0;
        let mut p: Vector = Vector::new(0.0, 0.0, 0.0);
        let mut n: Vector = Vector::new(0.0, 0.0, 0.0);
        for h in self {
            let (ht, hp, hn) = h.hit(r, t_min, closest);
            if ht > 0.0 {
                closest = ht;
                t = ht;
                p = hp;
                n = hn;
            }
        }
        (t, p, n)
    }
}
