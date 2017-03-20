use image::{DynamicImage,Luma};
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use num_cpus;

use geometry::{Viewable};
use scene::Scene;

const IMGX: u32 = 1000;
const IMGY: u32 = 1000;
const SCALE: f64 = 0.0025;

pub fn chunk(w: u32, h: u32, n: u32) -> Vec<Vec<(u32, u32)>> {
    let mut v = vec![];
    let size = (w + n - 1) / n;

    for i in 0..n {
        let mut pixels = vec![];

        for x in 0..size {
            let x = x + i * size;
            if x >= w { break; }

            for y in 0..h {
                pixels.push((x, y))
            }
        }

        v.push(pixels);
    }

    v
}

pub fn render(scene: Scene) -> DynamicImage {
    let mut image = DynamicImage::new_luma8(IMGX, IMGY);
    let cam = scene.camera.unwrap();
    let rays = Arc::new(cam.rays(IMGX, IMGY, SCALE));
    let tree = Arc::new(scene.tree);
    let chunks = num_cpus::get() as u32 * 4;

    let results: Result<Vec<Vec<(u32, u32, u8)>>, _> = chunk(IMGX, IMGY, chunks)
        .into_iter()
        .map(|chunk| {
            let tree = tree.clone();
            let rays = rays.clone();

            thread::spawn(move || {
                chunk.into_iter().map(|(x, y)| {
                    let ray = rays[(x * IMGX + y) as usize];
                    let min = tree.get_faces(ray).iter().fold(10000.0f64, |min, face| {
                        match face.intersects(ray) {
                            None => min,
                            Some(int) => min.min(ray.loc.distance_to(int))
                        }
                    });

                    let v = match min {
                        10000.0 => 0u8,
                        x => 255 - (255.0 / (x - 10.0)) as u8,
                    };

                   (x, y, v)
                })
                .collect()
            })
        })
        .collect::<Vec<JoinHandle<Vec<_>>>>()
        .into_iter()
        .map(|handle| handle.join())
        .collect();

    for result in results.unwrap().into_iter() {
        for (x, y, val) in result.into_iter() {
            let pixel = image.as_mut_luma8().unwrap().get_pixel_mut(x, y);
            *pixel = Luma([val]);
        };
    }

    image
}
