use std::cell::RefCell;
use std::borrow::Borrow;

use bounds::Bounds;
use triangle::Triangle;
use ray::Ray;

#[derive(Debug)]
pub struct Octree {
    depth: u8,
    bounds: Bounds,
    faces: Vec<Triangle>,
    trees: Vec<Octree>,
}

impl  Octree {

    pub fn new(bounds: Bounds, depth: u8) -> Octree {
        let mut trees = vec![];

        if depth == 0 {
            return Octree {
                depth: depth,
                bounds: bounds,
                faces: vec![],
                trees: trees,
            };
        }

        for i in 0..8 {
            let (x, y, z) = (i & 1, i >> 1 & 1, i >> 2 & 1);
            let (w, d, h) = (bounds.width() / 2.0, bounds.depth() / 2.0, bounds.height() / 2.0);
            let bounds = Bounds::new(
                bounds.xmin + w * x as f64,
                bounds.xmin + w * x as f64 + w,
                bounds.ymin + d * y as f64,
                bounds.ymin + d * y as f64 + d,
                bounds.zmin + h * z as f64,
                bounds.zmin + h * z as f64 + h,
            );

            trees.push(Octree::new(bounds, depth - 1));
        }

        Octree {
            depth: depth,
            bounds: bounds,
            trees: trees,
            faces: vec![],
        }
    }

    pub fn insert(&mut self, t: Triangle) {
        let space = t.bounds();

        if self.depth == 0 {
            self.faces.push(t);
        }

        for mut tree in self.trees.iter_mut() {
            if tree.bounds.overlaps(space) {
                tree.insert(t);
            }
        }
    }

    pub fn get_faces(&self, ray: Ray) -> Vec<Triangle> {
        let intersects = self.bounds.intersects(ray);
        let mut all: Vec<Triangle> = vec![];

        if self.depth == 0 && intersects {
            return self.faces.clone();
        }

        if intersects {
            for tree in self.trees.iter() {
                for tri in tree.get_faces(ray).iter() {
                    all.push(*tri);
                }
            }
        }

        all
    }
}

#[cfg(test)]
mod test {
    use octree::Octree;
    use bounds::Bounds;

    #[test]
    fn test_creation() {
        let bounds = Bounds::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let tree = Octree::new(bounds, 1);
        println!("{:?}", tree);

        assert!(false);
    }
}
