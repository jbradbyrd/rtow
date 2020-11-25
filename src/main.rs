fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;
    let float_width = image_width as f64;
    let float_height = image_height as f64;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining {}", j);
        for i in 0..image_width {
            let r = (i as f64) / float_width;
            let g = (j as f64) / float_height;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\nComplete!");
}
