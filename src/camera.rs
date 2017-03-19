use point::Point;
use vector::Vector;
use ray::Ray;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OrthoCamera {
    loc: Point,
    dir: Vector,
}

impl OrthoCamera {

    pub fn new(loc: Point, dir: Vector) -> OrthoCamera {
        OrthoCamera {
            loc: loc,
            dir: dir.to_unit(),
        }
    }

    pub fn rays(&self, w: u32, h: u32, z: f64) -> Vec<Ray> {
        let mut rays: Vec<Ray> = vec![];
        let up = Vector::new(0.0,0.0,1.0);
        let par = self.dir.cross(up);

        let offset_u = (w as f64 - 1.0) / 2.0;
        let offset_v = (h as f64 - 1.0) / 2.0;

        for u in 0..w {
            for v in 0..h {
                rays.push(Ray::new(
                    self.loc + Point::new(
                        /* x */ (u as f64 - offset_u) * z * par.x,
                        /* y */ (u as f64 - offset_u) * z * par.y,
                        /* z */ -(v as f64 - offset_v) * z,
                    ),
                    self.dir,
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

        println!("{:?}", camera.rays(2, 2, 1.0));
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

    #[test]
    fn test_more_rays() {
        let camera = OrthoCamera::new(
            Point::new(5.0, 5.0, 0.0),
            Vector::new(-1.0, -1.0, 0.0),
        );

        println!("{:?}", camera.rays(2, 2, 1.0));
        assert!(
            camera.rays(2, 2, 1.0) ==
            vec![

            ]
        )
    }
}
