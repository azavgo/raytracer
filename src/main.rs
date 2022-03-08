use std::fs::write;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable; 

fn hit_sphere(centre: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - centre;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let disciminant = b * b - 4.0 * a * c;
    if disciminant < 0.0 {
        -1.0
    } else {
        (-b - disciminant.sqrt()) / (2.0 * a)
    }
}

fn colour(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn ppm_image(image_width: i32, image_height: i32) -> String {
    let mut ppm_image = format!("P3\n{} {}\n255\n", image_width, image_height);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut ir: i32;
    let mut ig: i32;
    let mut ib: i32;

    let mut u: f64;
    let mut v: f64;

    let mut col: Vec3;
    let mut r: Ray;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            u = (i as f64) / (image_width as f64);
            v = (j as f64) / (image_height as f64);

            r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            col = colour(r);

            ir = (255.99 * col.r()) as i32;
            ig = (255.99 * col.g()) as i32;
            ib = (255.99 * col.b()) as i32;

            ppm_image += &format!("{} {} {}\n", ir, ig, ib);
        }
    }
    ppm_image
}

fn main() {
    write("output.ppm", ppm_image(200, 100)).unwrap();
}
