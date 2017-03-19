

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {

    pub fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord {
            x: x,
            y: y,
            z: z,
        }
    }
}
