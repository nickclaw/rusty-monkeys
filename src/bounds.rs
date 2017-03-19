
pub struct Bounds {
    pub xmin: i32,
    pub xmax: i32,
    pub ymin: i32,
    pub ymax: i32,
    pub zmin: i32,
    pub zmax: i32,
}

impl Bounds {

    pub fn new(x1: f64, x2: f64, y1: f64, y2: f64, x3: f64, x4: f64) -> Bounds {
        Bounds {
            xmin: x1.floor() as i32,
            xmax: x2.ceil() as i32,
            ymin: y1.floor() as i32,
            ymax: y2.ceil() as i32,
            zmin: y1.floor() as i32,
            zmax: y2.ceil() as i32,
        }
    }
}
