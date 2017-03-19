use std::iter::FromIterator;

use geometry::{Bounded, Viewable};
use bounds::Bounds;
use ray::Ray;

#[derive(Debug)]
pub struct Octree<T> where T: Bounded + Viewable + Copy {
    depth: u8,
    bounds: Bounds,
    faces: Vec<T>,
    trees: Vec<Octree<T>>,
}

impl<T> Octree<T> where T: Bounded + Viewable + Copy {

    pub fn new(depth: u8, bounds: Bounds) -> Octree<T> {
        Octree {
            depth: depth,
            bounds: bounds,
            faces: vec![],
            trees: vec![],
        }
    }

    pub fn insert(&mut self, t: T) {
        // case: leaf
        if self.depth == 0 {
            self.faces.push(t);
            return;
        }

        // case: not branched
        if self.trees.len() == 0 {
            self.subdivide();
            self.insert(t);
            return;
        }

        // case: add
        for mut tree in self.trees.iter_mut() {
            if tree.overlaps(t.bounds()) {
                tree.insert(t);
            }
        }
    }

    pub fn get_faces(&self, ray: Ray) -> Vec<T> {
        let intersects = self.bounds.intersects(ray);
        let mut all: Vec<T> = vec![];

        if self.depth == 0 && intersects {
            return self.faces.clone();
        }

        if intersects {
            for tree in self.trees.iter() {
                for t in tree.get_faces(ray).iter() {
                    all.push(*t);
                }
            }
        }

        all
    }

    fn subdivide(&mut self) {
        let bounds = self.bounds;
        self.trees = (0..8)
            .map(|i| {
                let (x, y, z) = (i & 1, i >> 1 & 1, i >> 2 & 1);
                let (w, d, h) = (bounds.width() / 2.0, bounds.depth() / 2.0, bounds.height() / 2.0);

                Octree::new(
                    self.depth - 1,
                    Bounds::new(
                        bounds.xmin + w * x as f64,
                        bounds.xmin + w * x as f64 + w,
                        bounds.ymin + d * y as f64,
                        bounds.ymin + d * y as f64 + d,
                        bounds.zmin + h * z as f64,
                        bounds.zmin + h * z as f64 + h,
                    )
                )
            })
            .collect::<Vec<Octree<T>>>();
    }

    fn overlaps(&self, b: Bounds) -> bool {
        self.bounds.overlaps(b)
    }
}

impl<T> FromIterator<T> for Octree<T> where T: Bounded + Viewable + Copy {
    fn from_iter<I>(iter: I) -> Self  where I: IntoIterator<Item=T> {
        let items: Vec<T> = iter.into_iter().collect();
        let bounds = items.iter().fold(Bounds::zero(), |sum, t| sum + t.bounds());
        let len = items.len();
        let depth = (1.2 * (len as f64).log(8.0)).round() as u8;
        let mut tree = Octree::new(depth, bounds);

        for item in items.into_iter() {
            tree.insert(item);
        }

        tree
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
