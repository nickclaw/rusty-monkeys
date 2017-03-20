#[macro_use]
extern crate image;
extern crate num_cpus;

use std::fs::File;

use scene::Scene;
use camera::OrthoCamera;
use point::Point;
use vector::Vector;
use render::render;

mod point;
mod vector;
mod bounds;
mod geometry;
mod triangle;
mod ray;
mod octree;
mod camera;
mod scene;
mod render;

fn main() {
    let src = "/Users/nickclaw/workspace/rust/raytracer/data/verts.obj";
    let dest = "/Users/nickclaw/workspace/rust/raytracer/out.png";
    let input = File::open(src).unwrap();
    let mut scene = Scene::from_file(input);

    scene.set_camera(OrthoCamera::new(
        Point::new(10.0, 10.0, 0.0),
        Vector::new(-1.0, -1.0, 0.0),
    ));

    scene.add_light(Point::new(0.0, 0.0, 10.0));
    scene.add_light(Point::new(10.0, 10.0, 10.0));

    let image = render(scene);
    image.as_luma8().unwrap().save(dest).unwrap();
}
