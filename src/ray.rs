use point::Point;
use vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub loc: Point,
    pub dir: Vector,
    pub inv: Vector,
}

impl Ray {
    pub fn new(loc: Point, dir: Vector) -> Ray {
        Ray {
            loc: loc,
            dir: dir.to_unit(),
            inv: dir.invert(),
        }
    }
}
