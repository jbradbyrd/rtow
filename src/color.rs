pub type Color = crate::vec3::Vec3;

pub fn write_color(
    stream: &mut dyn std::io::Write,
    pixel_color: Color,
) -> Result<(), std::io::Error> {
    // Write the translated [0,255] value of each color component.
    writeln!(
        stream,
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    )
}
