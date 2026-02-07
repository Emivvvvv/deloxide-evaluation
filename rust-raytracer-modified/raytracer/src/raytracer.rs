use crate::test_harness::{Arc, Mutex};

use image::png::PNGEncoder;
use image::ColorType;
use palette::Pixel;
use palette::Srgb;
use rand::Rng;
// use rayon::prelude::*; // Rayon not used in new render, but might be used elsewhere?
// render_line uses rand, not rayon.
// I'll keep rayon import if it was there, effectively harmless, but commented out if I want to be clean.
use std::fs::File;
use std::time::Instant;

use crate::config::Config;
use crate::materials::Material;
use crate::materials::Scatterable;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[cfg(test)]
use std::fs;

#[cfg(test)]
use crate::point3d::Point3D;

#[cfg(test)]
use crate::camera::Camera;
#[cfg(test)]
use crate::config::Sky;
#[cfg(test)]
use crate::materials::Lambertian;
#[cfg(test)]
use crate::materials::Light;

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

fn hit_world<'material>(
    world: &'material Vec<Sphere>,
    r: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

fn clamp(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn ray_color(
    ray: &Ray,
    scene: &Config,
    lights: &Vec<Sphere>,
    max_depth: usize,
    depth: usize,
) -> Srgb {
    let mut rng = rand::thread_rng();

    if depth == 0 {
        return Srgb::new(0.0, 0.0, 0.0);
    }
    let hit = hit_world(&scene.objects, ray, 0.001, f64::MAX);
    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(ray, &hit_record);
            match scattered {
                Some((scattered_ray, albedo)) => {
                    let mut light_red = 0.0;
                    let mut light_green = 0.0;
                    let mut light_blue = 0.0;
                    let mut prob = 0.1;
                    if let Material::Glass(_) = hit_record.material {
                        prob = 0.05;
                    }
                    if !lights.is_empty()
                        && rng.gen::<f64>() > (1.0 - lights.len() as f64 * prob)
                        && depth > (max_depth - 2)
                    {
                        for light in lights {
                            let light_ray =
                                Ray::new(hit_record.point, light.center - hit_record.point);
                            let target_color = ray_color(&light_ray, scene, lights, 2, 1);
                            light_red += albedo.red * target_color.red;
                            light_green += albedo.green * target_color.green;
                            light_blue += albedo.blue * target_color.blue;
                        }
                        light_red /= lights.len() as f32;
                        light_green /= lights.len() as f32;
                        light_blue /= lights.len() as f32;
                    }
                    match scattered_ray {
                        Some(sr) => {
                            let target_color = ray_color(&sr, scene, lights, max_depth, depth - 1);
                            Srgb::new(
                                clamp(light_red + albedo.red * target_color.red),
                                clamp(light_green + albedo.green * target_color.green),
                                clamp(light_blue + albedo.blue * target_color.blue),
                            )
                        }
                        None => albedo,
                    }
                }
                None => {
                    // don't bother bouncing absorbed rays towards lights
                    // (they would be absorbed in the opposite direction).
                    Srgb::new(0.0, 0.0, 0.0)
                }
            }
        }
        None => {
            let t: f32 = clamp(0.5 * (ray.direction.unit_vector().y() as f32 + 1.0));
            let u: f32 = clamp(0.5 * (ray.direction.unit_vector().x() as f32 + 1.0));
            match &scene.sky {
                None => Srgb::new(0.0, 0.0, 0.0),
                Some(sky) => match &sky.texture {
                    None => Srgb::new(
                        (1.0 - t) * 1.0 + t * 0.5,
                        (1.0 - t) * 1.0 + t * 0.7,
                        (1.0 - t) * 1.0 + t * 1.0,
                    ),
                    Some((pixels, width, height, _)) => {
                        let x = (u * (*width - 1) as f32) as usize;
                        let y = ((1.0 - t) * (*height - 1) as f32) as usize;
                        let pixel_red = &pixels[(y * *width + x) * 3];
                        let pixel_green = &pixels[(y * *width + x) * 3 + 1];
                        let pixel_blue = &pixels[(y * *width + x) * 3 + 2];
                        Srgb::new(
                            0.7 * *pixel_red as f32 / 255.0,
                            0.7 * *pixel_green as f32 / 255.0,
                            0.7 * *pixel_blue as f32 / 255.0,
                        )
                    }
                },
            }
        }
    }
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    let scene = Config {
        width: 80,
        height: 60,
        samples_per_pixel: 1,
        max_depth: 2,
        sky: Some(Sky::new_default_sky()),
        camera: Camera::new(
            Point3D::new(0.0, 0.0, -3.0),
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
            20.0,
            1.333,
        ),
        objects: Vec::new(),
    };
    let l = Vec::new();
    assert_eq!(ray_color(&r, &scene, &l, 2, 2), Srgb::new(0.75, 0.85, 1.0));
}

fn render_segment(
    pixels: &mut [u8],
    scene: &Config,
    lights: &Vec<Sphere>,
    y: usize,
    x_offset: usize,
) {
    let mut rng = rand::thread_rng();

    let bounds = (scene.width, scene.height);
    let width = pixels.len() / 3;

    for x in 0..width {
        let global_x = x_offset + x;
        let mut pixel_colors: Vec<f32> = vec![0.0; 3];
        for _s in 0..scene.samples_per_pixel {
            let u = (global_x as f64 + rng.gen::<f64>()) / (bounds.0 as f64 - 1.0);
            let v = (bounds.1 as f64 - (y as f64 + rng.gen::<f64>())) / (bounds.1 as f64 - 1.0);
            let r = scene.camera.get_ray(u, v);
            let c = ray_color(&r, scene, lights, scene.max_depth, scene.max_depth);
            pixel_colors[0] += c.red;
            pixel_colors[1] += c.green;
            pixel_colors[2] += c.blue;
        }
        let scale = 1.0 / scene.samples_per_pixel as f32;
        let color = Srgb::new(
            (scale * pixel_colors[0]).sqrt(),
            (scale * pixel_colors[1]).sqrt(),
            (scale * pixel_colors[2]).sqrt(),
        );
        let pixel: [u8; 3] = color.into_format().into_raw();
        pixels[x * 3] = pixel[0];
        pixels[x * 3 + 1] = pixel[1];
        pixels[x * 3 + 2] = pixel[2];
    }
}

fn find_lights(world: &[Sphere]) -> Vec<Sphere> {
    world
        .iter()
        .filter(|s| matches!(s.material, Material::Light(_)))
        .cloned()
        .collect()
}

#[test]
fn test_find_lights() {
    let world = vec![
        Sphere::new(
            Point3D::new(0.0, 0.0, -1.0),
            0.5,
            Material::Light(Light::new()),
        ),
        Sphere::new(
            Point3D::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Lambertian::new(Srgb::new(
                0.5 as f32, 0.5 as f32, 0.5 as f32,
            ))),
        ),
    ];
    assert_eq!(find_lights(&world).len(), 1);
}

pub fn render(filename: &str, scene: Config) {
    let image_width = scene.width;
    let image_height = scene.height;

    let buffer_size = image_width * image_height * 3;

    #[cfg(any(
        feature = "deloxide",
        feature = "stress_random",
        feature = "stress_component",
        feature = "stress_aggressive",
        feature = "stress_gentle",
        feature = "deloxide_lock_order",
        feature = "parking_lot_deadlock"
    ))]
    let _deadlock_detector = crate::test_harness::init_detectors();

    let final_image = Arc::new(Mutex::new(vec![0u8; buffer_size]));
    let lock_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let lights = find_lights(&scene.objects);
    let start = Instant::now();

    let chunk_size = 16;
    let chunks_x = image_width.div_ceil(chunk_size);
    let chunks_y = image_height.div_ceil(chunk_size);
    let total_chunks = chunks_x * chunks_y;
    let next_chunk = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    // Determine thread count
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    std::thread::scope(|s| {
        let mut handles = vec![];

        for _ in 0..num_threads {
            let img_clone = final_image.clone();
            let lock_count = lock_count.clone();
            let next_chunk = next_chunk.clone();
            let scene_ref = &scene;
            let lights_ref = &lights;

            handles.push(s.spawn(move || {
                loop {
                    let chunk_idx = next_chunk.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if chunk_idx >= total_chunks {
                        break;
                    }

                    let chunk_y = chunk_idx / chunks_x;
                    let chunk_x = chunk_idx % chunks_x;

                    let y_start = chunk_y * chunk_size;
                    let x_start = chunk_x * chunk_size;

                    // Clamp to image bounds (though loop limits inside render_segment/logic handle width, we must be careful with y)
                    let y_end = std::cmp::min(y_start + chunk_size, image_height);

                    // Iterate pixels in this chunk
                    // Note: render_segment handles x iteration for a given y.
                    // We need to iterate y lines in this chunk.

                    for y in y_start..y_end {
                        let actual_chunk_width = std::cmp::min(chunk_size, image_width - x_start);
                        let mut chunk_buffer = vec![0u8; actual_chunk_width * 3];

                        render_segment(&mut chunk_buffer, scene_ref, lights_ref, y, x_start);

                        // Critical Section
                        lock_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                        let start_index = (y * image_width + x_start) * 3;
                        let end_index = start_index + chunk_buffer.len();

                        {
                            let mut guard = crate::lock!(img_clone);
                            guard[start_index..end_index].copy_from_slice(&chunk_buffer);
                        }
                    }
                }
            }));
        }

        // Wait is implicit in scope
    });

    println!("Frame time: {}ms", start.elapsed().as_millis());
    println!(
        "Total Lock Acquisitions: {}",
        lock_count.load(std::sync::atomic::Ordering::Relaxed)
    );

    let pixels = {
        {
            #[cfg(any(feature = "use_std", feature = "no_deadlocks"))]
            {
                Arc::try_unwrap(final_image)
                    .ok()
                    .expect("Arc still has owners")
                    .into_inner()
                    .unwrap()
            }
            #[cfg(not(any(feature = "use_std", feature = "no_deadlocks")))]
            {
                Arc::try_unwrap(final_image)
                    .ok()
                    .expect("Arc still has owners")
                    .into_inner()
            }
        }
    };

    write_image(filename, &pixels, (image_width, image_height)).expect("error writing image");
}

#[test]
fn test_render_full_test_scene() {
    let json = fs::read("data/test_scene.json").expect("Unable to read file");
    let mut scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    scene.width = 80;
    scene.height = 60;
    render("/tmp/test_scene.png", scene);
}

#[test]
fn test_render_full_cover_scene() {
    let json = fs::read("data/cover_scene.json").expect("Unable to read file");
    let mut scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    scene.width = 40;
    scene.height = 30;
    render("/tmp/cover_scene.png", scene);
}
