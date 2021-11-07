#![allow(dead_code)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use material::*;
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
        if let Some((attenuation, scattered)) = rec.mat_ptr.unwrap().scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::zero();
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn random_scene<'a>() -> HittableList<'a> {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_unit_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_unit_double(),
                0.2,
                b as f64 + 0.9 * random_unit_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit() * Color::random_unit();
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 10.0;
    let image_width = 1280 * 2;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
