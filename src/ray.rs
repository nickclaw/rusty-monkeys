use point::Point;
use vector::Vector;

#[derive(Debug, PartialEq)]
pub struct Ray {
    loc: Point,
    dir: Vector,
}

impl Ray {
    pub fn new(loc: Point, dir: Vector) -> Ray {
        Ray {
            loc: loc,
            dir: dir.to_unit(),
        }
    }
}
