extern crate image;
extern crate num_cpus;

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::collections::HashMap;

use geometry::Viewable;
use triangle::Triangle;
use point::Point;
use camera::OrthoCamera;
use octree::Octree;
use bounds::Bounds;

const PATH:&'static str = "/Users/nickclaw/workspace/rust/raytracer/out.png";
const IMGX: u32 = 250;
const IMGY: u32 = 250;

#[derive(Debug)]
pub struct Scene {
    tree: Octree<Triangle>,
}

impl Scene{

    pub fn from_file(path: &str) -> Result<Scene, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut verts: Vec<Point> = vec![];
        let mut objects: Vec<Triangle> = vec![];

        for line in reader.lines().map(|l| l.unwrap()) {
            match line.chars().next().unwrap() {
                'o' => verts = vec![],
                'v' => verts.push(Point::from_str(&line)),
                'f' => objects.push(Triangle::from_str(&line, &verts)),
                _ => continue, // choosing not to parse other types
            }
        }

        Ok(Scene { tree: objects.into_iter().collect() })
    }

    pub fn render(self, camera: OrthoCamera) -> Result<bool, Error> {
        let mut image = image::ImageBuffer::new(IMGX, IMGY);
        let rays = Arc::new(camera.rays(IMGX, IMGY, 0.01));
        let tree = Arc::new(self.tree);

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
                            let hits = tree.get_faces(ray).iter().any(|face| face.intersects(ray));
                            let v = match hits {
                                true =>  255u8,
                                false =>  0u8,
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
                let pixel = image.get_pixel_mut(x, y);
                *pixel = image::Luma([val]);
            };
        }

        // write it out to a file
        image.save(PATH).unwrap();
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use scene::Scene;

    #[test]
    fn test_simple() {
        let path = "/Users/nickclaw/workspace/rust/raytracer/data/triangle.obj";
        let scene = Scene::from_file(path);
        assert!(scene.is_ok());
    }

    #[test]
    fn test_complex() {
        let path = "/Users/nickclaw/workspace/rust/raytracer/data/monkey.obj";
        let scene = Scene::from_file(path);
        println!("{:?}", scene);
        assert!(scene.is_ok());
    }
}
