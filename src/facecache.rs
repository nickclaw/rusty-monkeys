use std::collections::HashMap;
use std::collections::HashSet;

use coord::Coord;
use triangle::Triangle;

struct FaceCache<'a> {
    size: u64,
    cache: HashMap<Coord, Vec<&'a Triangle>>,
}

impl <'a> FaceCache<'a> {

    pub fn new(size: u64) -> FaceCache<'a> {
        FaceCache {
            size: size,
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tri: &'a Triangle) {
        let bounds = tri.into_bounds();

        for i in bounds.xmin..(bounds.xmax+1) {
            for j in bounds.ymin..(bounds.ymax+1) {
                for k in bounds.zmin..(bounds.zmax+1) {
                    let coord = Coord::new(i, j, j);
                    let mut set = self.cache.entry(coord).or_insert(Vec::new());
                    set.push(&tri);
                }
            }
        }
    }
}
