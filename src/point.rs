use std::ops::{Sub,Add};
use vector::Vector;
use std::str::FromStr;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_str(line: &str) -> Point {
        let mut entries = line.split_whitespace();
        entries.next();

        let x = f64::from_str(entries.next().unwrap()).unwrap();
        let y = f64::from_str(entries.next().unwrap()).unwrap();
        let z = f64::from_str(entries.next().unwrap()).unwrap();

        Point::new(x, y, z)
    }

    pub fn distance_to(self, p: Point) -> f64 {
        let diff = self - p;
        let sum = diff.x.powi(2) + diff.y.powi(2) + diff.z.powi(2);
        sum.sqrt()
    }

    pub fn vector_to(self, p: Point) -> Vector {
        let diff = self - p;
        Vector::new(
            diff.x,
            diff.y,
            diff.z,
        )
    }

    pub fn translate(self, v: Vector) -> Point {
        Point::new(
            self.x + v.x,
            self.y + v.y,
            self.z + v.z,
        )
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }
}


#[cfg(test)]
mod test {
    use point::Point;

    #[test]
    fn test_distance() {
        // basic 3,4,5 triangle
        let a = Point::new(0.0, 3.0, 0.0);
        let b = Point::new(4.0, 0.0, 0.0);
        assert!(a.distance_to(b) == 5.0);
    }

    #[test]
    fn test_add() {
        assert!(Point::new(1.0,2.0,3.0) + Point::new(1.0,2.0,3.0) == Point::new(2.0,4.0,6.0));
        assert!(Point::new(0.0,0.0,0.0) + Point::new(-1.0,-1.0,-1.0) == Point::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_sub() {
        assert!(Point::new(1.0,2.0,3.0) - Point::new(1.0,2.0,3.0) == Point::new(0.0,0.0,0.0));
        assert!(Point::new(0.0,0.0,0.0) - Point::new(-1.0,-1.0,-1.0) == Point::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_eq() {
        assert!(Point::new(1.0,2.0,3.0) == Point::new(1.0,2.0,3.0));
        assert!(Point::new(1.0,2.0,3.0) != Point::new(3.0,2.0,1.0));
    }
}
