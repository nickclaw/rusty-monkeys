use point::Point;
use vector::Vector;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub loc: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(loc: Point, dir: Vector) -> Ray {
        Ray {
            loc: loc,
            dir: dir.to_unit(),
        }
    }
}
