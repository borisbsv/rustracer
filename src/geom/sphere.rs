use super::hittable::Hittable;
use super::ray::Ray;
use super::vec::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (f64, Vector, Vector) {
        let oc = r.or - self.center;
        let a = r.dir.dot(r.dir);
        let b = oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return (0.0, Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0));
        }

        let p: Vector;

        let compute_t = |f: fn(f64, f64) -> f64| f(-b, (b * b - a * c).sqrt()) / a;
        let t = compute_t(std::ops::Add::add);
        if t_max > t && t > t_min {
            p = r.at_param(t);
            return (t, p, (p - self.center).to_unit());
        }

        let t = compute_t(std::ops::Sub::sub);
        if t_max > t && t > t_min {
            p = r.at_param(t);
            return (t, p, (p - self.center).to_unit());
        }

        (0.0, Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0))
    }
}
