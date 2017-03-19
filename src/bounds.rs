use ray::Ray;

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

    pub fn width(&self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn depth(&self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn height(&self) -> f64 {
        self.zmax - self.zmin
    }

    pub fn overlaps(&self, other: &Bounds) -> bool {
        true
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        true
    }
}
