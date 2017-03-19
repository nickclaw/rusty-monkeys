extern crate image;

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::collections::HashMap;

use self::image::{ImageBuffer, Rgb};

use triangle::Triangle;
use point::Point;
use camera::OrthoCamera;

const PATH:&'static str = "/Users/nickclaw/workspace/rust/raytracer/out.png";
const IMGX: u32 = 250;
const IMGY: u32 = 250;

#[derive(Debug)]
pub struct Scene {
    faces: Vec<Triangle>,
}

impl Scene {

    pub fn from_file(path: &str) -> Result<Scene, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut verts: Vec<Point> = vec![];
        let mut faces: Vec<Triangle> = vec![];

        for line in reader.lines().map(|l| l.unwrap()) {
            match line.chars().next().unwrap() {
                'o' => verts = vec![],
                'v' => verts.push(Point::from_str(&line)),
                'f' => faces.push(Triangle::from_str(&line, &verts)),
                _ => continue, // choosing not to parse other types
            }
        }

        Ok(Scene { faces: faces })
    }

    pub fn render(self, camera: OrthoCamera) -> Result<bool, Error> {
        let mut image = image::ImageBuffer::new(IMGX, IMGY);
        let rays = Arc::new(camera.rays(IMGX, IMGY, 0.01));
        let faces = Arc::new(self.faces);
        let (tx, rx) = mpsc::channel();

        for x in 0..IMGX {
            let tx = tx.clone();
            let faces = faces.clone();
            let rays = rays.clone();

            thread::spawn(move || {
                let vals = (0..IMGY)
                    .map(|y| rays[(x * IMGX + y) as usize].clone())
                    .map(|ray| faces.iter().fold(None, |min: Option<f64>, face| {
                        let dist: Option<f64> = face.intersects(&ray);

                        match (min, dist) {
                           (Some(a), Some(b)) => Some(a.max(b)),
                           (_, Some(x)) => Some(x),
                           (_, _) => min,
                       }
                    }))
                    .map(|b| match b {
                        Some(x) => 255 - (255.0 / x) as u8,
                        None => 0,
                    })
                    .collect::<Vec<u8>>();

                tx.send((x, vals)).unwrap();
            });
        }

        let mut all: HashMap<u32, Vec<u8>> = HashMap::new();
        for i in 0..IMGX {
            let (i, row) = rx.recv().unwrap();
            all.insert(i, row);
        }

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let val = all.get(&x).unwrap()[y as usize];


            *pixel = image::Luma([val]);
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
