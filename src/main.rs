mod geom;

use geom::Hittable;
use geom::Ray;
use geom::Sphere;
use geom::Vector;

use std::fs::File;
use std::io::Write;

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vector {
    let (t, _p, n) = world.hit(r, 0.0, std::f64::MAX);

    if t > 0.0 {
        return Vector::new(n.x + 1.0, n.y + 1.0, n.z + 1.0).scale(0.5);
    }

    let u_dir = r.dir.to_unit();
    let t = 0.5 * (u_dir.y + 1.0);
    Vector::new(1.0, 1.0, 1.0).scale(1.0 - t) + Vector::new(0.5, 0.7, 1.0).scale(t)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut out = File::create("out.ppm").expect("Unable to read file");

    write!(out, "P3\n{} {}\n255\n", nx, ny).expect("Unable to write file");

    let llc = Vector::new(-2.0, -1.0, -1.0);
    let horizontal = Vector::new(4.0, 0.0, 0.0);
    let vertical = Vector::new(0.0, 2.0, 0.0);
    let origin = Vector::new(0.0, 0.0, 0.0);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, llc + horizontal.scale(u) + vertical.scale(v));
            let col = color(&r, &world);

            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;

            write!(out, "{} {} {}\n", ir, ig, ib).expect("Unable to write file");
        }
    }
}
