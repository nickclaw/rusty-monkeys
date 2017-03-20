use std::ops::Add;

use geometry::{Bounded, Viewable};
use ray::Ray;
use point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Bounds {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl Bounds {

    pub fn new(
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        zmin: f64,
        zmax: f64,
    ) -> Bounds {
        Bounds {
            xmin: xmin,
            xmax: xmax,
            ymin: ymin,
            ymax: ymax,
            zmin: zmin,
            zmax: zmax,
        }
    }

    pub fn zero() -> Bounds {
        Bounds::new(0.0,0.0,0.0,0.0,0.0,0.0)
    }

    pub fn width(self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn depth(self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn height(self) -> f64 {
        self.zmax - self.zmin
    }

    pub fn overlaps(self, other: Bounds) -> bool {
        if self.xmax < other.xmin { return false; }
        if self.xmin > other.xmax { return false; }
        if self.ymax < other.ymin { return false; }
        if self.ymin > other.ymax { return false; }
        if self.zmax < other.zmin { return false; }
        if self.zmin > other.zmax { return false; }
        true
    }
}

impl Add for Bounds {
    type Output = Bounds;

    fn add(self, p: Bounds) -> Bounds {
        Bounds::new(
            self.xmin.min(p.xmin),
            self.xmax.max(p.xmax),
            self.ymin.min(p.ymin),
            self.ymax.max(p.ymax),
            self.zmin.min(p.zmin),
            self.zmax.max(p.zmax),
        )
    }
}

impl Bounded for Bounds {
    fn bounds(&self) -> Bounds {
        self.clone()
    }
}

impl Viewable for Bounds {
    fn intersects(&self, r: Ray) -> Option<Point> {
        let _txmin = (self.xmin - r.loc.x) / r.dir.x;
        let _txmax = (self.xmax - r.loc.x) / r.dir.x;
        let (mut txmin, mut txmax) = if _txmin > _txmax { (_txmax, _txmin) } else { (_txmin, _txmax) };

        let _tymin = (self.ymin - r.loc.y) / r.dir.y;
        let _tymax = (self.ymax - r.loc.y) / r.dir.y;
        let (mut tymin, mut tymax) = if _tymin > _tymax { (_tymax, _tymin) } else { (_tymin, _tymax) };

        if (txmin > tymax) || (tymin > txmax) {
            return None;
        }

        if tymin > txmin {
            txmin = tymin;
        }

        if tymax < txmax {
            txmax = tymax;
        }

        let _tzmin = (self.zmin - r.loc.z) / r.dir.z;
        let _tzmax = (self.zmax - r.loc.z) / r.dir.z;
        let (mut tzmin, mut tzmax) = if _tzmin > _tzmax { (_tzmax, _tzmin) } else { (_tzmin, _tzmax) };

        if (txmin > tzmax) || (tzmin > txmax) {
            return None;
        }

        Some(Point::zero())
    }
}
