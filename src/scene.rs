extern crate image;

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

use self::image::{ImageBuffer, Rgb};

use triangle::Triangle;
use point::Point;
use camera::OrthoCamera;

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

    pub fn render(&self, camera: OrthoCamera) -> Result<bool, Error> {
        let path = "/Users/nickclaw/workspace/rust/raytracer/out.png";
        let ref faces = self.faces;
        let imgx = 250;
        let imgy = 250;
        let mut image = image::ImageBuffer::new(imgx, imgy);
        let rays = camera.rays(imgx, imgy, 0.02);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let ref ray = rays[(x * imgx + y) as usize];
            let intersects = faces.into_iter().any(|face| face.intersects(ray));
            let val: u8 = if intersects { 255 } else { 0 };

            *pixel = image::Luma([val]);
        }

        // write it out to a file
        image.save(path).unwrap();
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
