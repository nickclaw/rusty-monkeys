use std::ops::{Add,Sub,Mul,Div};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn mag(&self) -> f64 {
        let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        sum.sqrt()
    }

    pub fn to_unit(&self) -> Vector {
        let mag = self.mag();
        self.clone() / mag
    }

    pub fn cross(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x
            + self.y * v.y
            + self.z * v.z
    }

    pub fn is_orthogonal(&self, v: &Vector) -> bool {
        self.dot(v) == 0.0
    }
}

impl Add for Vector  {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Vector  {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, n: f64) -> Vector {
        Vector {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, n: f64) -> Vector {
        Vector {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

#[cfg(test)]
mod test {
    use vector::Vector;

    #[test]
    fn test_mag() {
        assert!(Vector::new(0.0,0.0,0.0).mag() == 0.0);
        assert!(Vector::new(1.0,0.0,0.0).mag() == 1.0);
        assert!(Vector::new(0.0,1.0,0.0).mag() == 1.0);
        assert!(Vector::new(0.0,0.0,1.0).mag() == 1.0);
    }

    #[test]
    fn test_unit() {
        assert!(Vector::new(2.0,0.0,0.0).to_unit() == Vector::new(1.0,0.0,0.0));
    }

    #[test]
    fn test_cross() {
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let z = Vector::new(0.0, 0.0, 1.0);

        assert!(x.cross(&y) == z);
    }

    #[test]
    fn test_dot() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(4.0, 5.0, 6.0);
        assert!(a.dot(&b) == 32.0);
    }

    #[test]
    fn test_ortho() {
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let z = Vector::new(0.0, 0.0, 1.0);

        assert!(x.is_orthogonal(&y));
        assert!(x.is_orthogonal(&z));
        assert!(y.is_orthogonal(&z));
    }

    #[test]
    fn test_add() {
        assert!(Vector::new(0.0,0.0,0.0) + Vector::new(0.0,0.0,0.0) == Vector::new(0.0,0.0,0.0));
        assert!(Vector::new(1.0,1.0,1.0) + Vector::new(1.0,1.0,1.0) == Vector::new(2.0,2.0,2.0));
        assert!(Vector::new(1.0,1.0,1.0) + Vector::new(-1.0,-1.0,-1.0) == Vector::new(0.0,0.0,0.0));
    }

    #[test]
    fn test_sub() {
        assert!(Vector::new(1.0,1.0,1.0) - Vector::new(1.0,1.0,1.0) == Vector::new(0.0,0.0,0.0));
    }

    #[test]
    fn test_div() {
        assert!(Vector::new(1.0,1.0,1.0) / 1.0 == Vector::new(1.0,1.0,1.0));
        assert!(Vector::new(1.0,1.0,1.0) / 0.5 == Vector::new(2.0,2.0,2.0));
        assert!(Vector::new(1.0,1.0,1.0) / -2.0 == Vector::new(-0.5,-0.5,-0.5));
    }

    #[test]
    fn test_mul() {
        assert!(Vector::new(1.0,1.0,1.0) * 1.0 == Vector::new(1.0,1.0,1.0));
        assert!(Vector::new(1.0,1.0,1.0) * 2.0 == Vector::new(2.0,2.0,2.0));
        assert!(Vector::new(1.0,1.0,1.0) * -0.5 == Vector::new(-0.5,-0.5,-0.5));
    }

    #[test]
    fn test_eq() {
        assert!(Vector::new(0.0,0.0,0.0) == Vector::new(0.0,0.0,0.0));
    }
}
