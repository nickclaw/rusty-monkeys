use point::Point;
use vector::Vector;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    v0: Point,
    v1: Point,
    v2: Point,
    n: Vector,
}

impl Triangle {

    pub fn new(v0: Point, v1: Point, v2: Point) -> Triangle {
        let e1 = v1.vector_to(&v0);
        let e2 = v2.vector_to(&v0);
        let n = e1.cross(&e2).to_unit();

        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            n: n,
        }
    }
}

#[cfg(test)]
mod test {
    use triangle::Triangle;
    use point::Point;
    use vector::Vector;

    #[test]
    fn test_create() {
        //  y
        //  (0, 1, 0)
        //  |\
        //  |  \
        //  |____\       x
        //(0,0,0)  (1, 0, 0)
        let tri = Triangle::new(
            Point::new(0.0,0.0,0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0)
        );

        // counter-clockwise vectors means the face is pointing towards us
        assert!(tri.n == Vector::new(0.0, 0.0, 1.0), "face is pointing up");
    }
}