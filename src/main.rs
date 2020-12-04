use ray::Ray;
use vec3::Vec3;

mod camera;
mod ray;
mod vec3;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0_f32, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    let mut pixels = Vec::with_capacity((image_width * image_height * 3) as usize);
    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f32 / (image_width - 1) as f32;
            let v = y as f32 / (image_height - 1) as f32;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let pixel_color = ray_color(&r);
            let r = (255.999 * pixel_color.x) as u8;
            let g = (255.999 * pixel_color.y) as u8;
            let b = (255.999 * pixel_color.z) as u8;
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
        }
    }

    image::save_buffer(
        "image.png",
        &pixels,
        image_width as u32,
        image_height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0)
}
