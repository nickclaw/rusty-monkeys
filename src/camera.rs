use point::Point;
use vector::Vector;
use ray::Ray;

#[derive(Debug, PartialEq)]
pub struct OrthoCamera {
    loc: Point,
    dir: Vector,
}

impl OrthoCamera {

    pub fn new(loc: Point, dir: Vector) -> OrthoCamera {
        OrthoCamera {
            loc: loc,
            dir: dir,
        }
    }

    pub fn rays(&self, w: usize, h: usize, z: f64) -> Vec<Ray> {
        let mut rays: Vec<Ray> = vec![];
        let offset_u = (w as f64 - 1.0) / 2.0;
        let offset_v = (h as f64 - 1.0) / 2.0;

        for u in 0..w {
            for v in 0..h {
                rays.push(Ray::new(
                    self.loc.clone() - Point::new(
                        (u as f64 - offset_u) * z * self.dir.y,
                        (u as f64 - offset_u) * z * self.dir.x,
                        (v as f64 - offset_v) * z,
                    ),
                    self.dir.clone(),
                ));
            }
        }

        rays
    }
}

#[cfg(test)]
mod test {
    use camera::OrthoCamera;
    use point::Point;
    use vector::Vector;
    use ray::Ray;

    #[test]
    fn test_simple_rays() {
        // pointing towards -x
        let camera = OrthoCamera::new(
            Point::new(10.0, 0.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
        );

        assert!(
            camera.rays(2, 2, 1.0) ==
            vec![
                Ray::new(Point::new(10.0,-0.5,0.5), Vector::new(-1.0, 0.0, 0.0)),
                Ray::new(Point::new(10.0,-0.5,-0.5), Vector::new(-1.0, 0.0, 0.0)),
                Ray::new(Point::new(10.0,0.5,0.5), Vector::new(-1.0, 0.0, 0.0)),
                Ray::new(Point::new(10.0,0.5,-0.5), Vector::new(-1.0, 0.0, 0.0)),
            ]
        );
    }
}
