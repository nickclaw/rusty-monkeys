use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use triangle::Triangle;
use point::Point;
use octree::Octree;
use camera::OrthoCamera;

pub struct Scene {
    pub camera: Option<OrthoCamera>,
    pub lights: Vec<Point>,
    pub tree: Octree<Triangle>,
}

impl Scene {

    pub fn from_file(file: File) -> Scene {
        let reader = BufReader::new(file);
        let mut verts: Vec<Point> = vec![];
        let mut objects: Vec<Triangle> = vec![];

        for line in reader.lines().map(|l| l.unwrap()) {
            match line.chars().next().unwrap() {
                'v' => verts.push(Point::from_str(&line)),
                'f' => objects.push(Triangle::from_str(&line, &verts)),
                _ => continue, // choosing not to parse other types
            }
        }

        Scene {
            camera: None,
            lights: vec![],
            tree: objects.into_iter().collect(),
        }
    }

    pub fn set_camera(&mut self, cam: OrthoCamera) {
        self.camera = Some(cam);
    }

    pub fn add_light(&mut self, p: Point) {
        self.lights.push(p);
    }
}
