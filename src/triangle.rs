use std::str::FromStr;

use geometry::{Bounded,Viewable};
use point::Point;
use vector::Vector;
use ray::Ray;
use bounds::Bounds;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub n: Vector,
}

impl Triangle {

    pub fn new(v0: Point, v1: Point, v2: Point) -> Triangle {
        let e1 = v1.vector_to(v0);
        let e2 = v2.vector_to(v0);
        let n = e1.cross(e2).to_unit();

        Triangle {
            v0: v0,
            v1: v1,
            v2: v2,
            n: n,
        }
    }

    pub fn from_str(line: &str, verts: &Vec<Point>) -> Triangle {
        let mut entries = line.split_whitespace();
        entries.next();

        let v0 = verts[usize::from_str(entries.next().unwrap()).unwrap() - 1];
        let v1 = verts[usize::from_str(entries.next().unwrap()).unwrap() - 1];
        let v2 = verts[usize::from_str(entries.next().unwrap()).unwrap() - 1];

        Triangle::new(v0, v1, v2)
    }
}

impl Bounded for Triangle {
    fn bounds(&self) -> Bounds {
        Bounds::new(
            self.v0.x.min(self.v1.x.min(self.v2.x)),
            self.v0.x.max(self.v1.x.max(self.v2.x)),
            self.v0.y.min(self.v1.y.min(self.v2.y)),
            self.v0.y.max(self.v1.y.max(self.v2.y)),
            self.v0.z.min(self.v1.z.min(self.v2.z)),
            self.v0.z.max(self.v1.z.max(self.v2.z)),
        )
    }
}

impl Viewable for Triangle {
    fn intersects(&self, ray: Ray) -> Option<Point> {
        let ev1 = self.v1.vector_to(self.v0);
        let ev2 = self.v2.vector_to(self.v0);
        let pvec = ray.dir.cross(ev2);
        let det = ev1.dot(pvec);

        if det > -0.0001 && det < 0.0001 {
            return None;
        }

        let invdet = 1.0 / det;
        let tvec = ray.loc.vector_to(self.v0);

        let u = tvec.dot(pvec) * invdet;
        if u < 0.0 || u > 1.0  {
            return None;
        }


        let qvec = tvec.cross(ev1);
        let v = ray.dir.dot(qvec) * invdet;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = ev2.dot(qvec) * invdet;

        Some(ray.loc.translate(ray.dir * t))
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
