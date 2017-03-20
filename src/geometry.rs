use bounds::Bounds;
use ray::Ray;
use point::Point;

pub trait Bounded {
    fn bounds(&self) -> Bounds;
}

pub trait Viewable {
    fn intersects(&self, r: Ray) -> Option<Point>;
}
