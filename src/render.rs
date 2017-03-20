use image::{DynamicImage,Luma};
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use num_cpus;

use geometry::{Viewable};
use scene::Scene;

const IMGX: u32 = 250;
const IMGY: u32 = 250;

pub fn render(scene: Scene) -> DynamicImage {
    let mut image = DynamicImage::new_luma8(IMGX, IMGY);
    let cam = scene.camera.unwrap();
    let rays = Arc::new(cam.rays(IMGX, IMGY, 0.01));
    let tree = Arc::new(scene.tree);

    let chunks = num_cpus::get() as u32 * 4;
    let chunk_size = (IMGX + chunks - 1) / chunks;

    let results: Result<Vec<Vec<(u32, u32, u8)>>, _> = (0..chunks)
        .map(|step| {
            let tree = tree.clone();
            let rays = rays.clone();

            thread::spawn(move || {
                let mut vals: Vec<(u32, u32, u8)> = vec![];

                for x in 0..chunk_size {
                    let x = x + step * chunk_size;

                    if x >= IMGX {
                        break;
                    }

                    for y in 0..IMGY {
                        let ray = rays[(x * IMGX + y) as usize];
                        let min = tree.get_faces(ray)
                            .iter()
                            .filter(|face| face.n.dot(ray.dir) < 0.0)
                            .fold(10000.0f64, |min, face| {
                                match face.intersects(ray) {
                                    None => min,
                                    Some(int) => min.min(ray.loc.distance_to(int))
                                }
                            });

                        let v = match min {
                            10000.0 => 0u8,
                            x => 255 - (255.0 / (x - 10.0)) as u8,
                        };

                       vals.push((x, y, v))
                    }
                }

                vals
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
