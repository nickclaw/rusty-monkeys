use std::fs::File;

use scene::Scene;
use camera::OrthoCamera;
use point::Point;
use vector::Vector;

mod point;
mod vector;
mod triangle;
mod ray;
mod camera;
mod scene;

fn main() {
    let path = "/Users/nickclaw/workspace/rust/raytracer/data/monkey.obj";
    let scene = Scene::from_file(path).unwrap();
    let camera = OrthoCamera::new(
        Point::new(10.0, 10.0, 0.0),
        Vector::new(-1.0, -1.0, 0.0),
    );

    scene.render(camera).unwrap();
}
