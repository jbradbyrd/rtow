#![allow(dead_code)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use ray::*;
use rayon::prelude::*;
use rtweekend::*;
use sphere::*;
use std;
use std::io;
use std::io::Write;
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        // lambert
        let target = rec.p + rec.normal + random_unit_vector();
        // hemispherical
        //let target = rec.p + random_in_hemisphere(rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    let mut scanline = Vec::<Color>::new();
    scanline.reserve_exact(image_width);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining {:<5}", j);
        io::stderr().flush().unwrap();

        (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::default();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random_unit_double()) / (image_width - 1) as f64;
                    let v = (j as f64 + random_unit_double()) / (image_height - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, max_depth);
                }
                pixel_color
            })
            .collect_into_vec(&mut scanline);

        for pixel in &scanline {
            write_color(&mut io::stdout(), *pixel, samples_per_pixel).unwrap();
        }
    }

    eprintln!("\nDone.");
}
