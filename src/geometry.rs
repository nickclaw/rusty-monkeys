use bounds::Bounds;
use ray::Ray;

pub trait Bounded {
    fn bounds(&self) -> Bounds;
}

pub trait Viewable {
    fn intersects(&self, r: Ray) -> bool;
}
