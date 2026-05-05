use super::vector::VectorSpace;
use num::{Num, ToPrimitive};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T: Num + ToPrimitive + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Num + ToPrimitive + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: Num + ToPrimitive + Copy> VectorSpace<T> for Vec3<T> {
    type CrossOutput = Vec3<T>;
    type CastType = T;

    fn zero() -> Self {
        unimplemented!()
    }

    fn one() -> Self {
        unimplemented!()
    }

    fn up() -> Self {
        unimplemented!()
    }

    fn right() -> Self {
        unimplemented!()
    }

    fn parse(self) -> Option<Self::CastType> {
        _ = self.x + self.y + self.z;
        unimplemented!()
    }

    fn magnitude(self) -> T {
        unimplemented!()
    }

    fn invert(self) -> Self {
        unimplemented!()
    }

    fn add(self, other: Self) -> Self {
        _ = other;
        unimplemented!()
    }

    fn dot(self, other: Self) -> T {
        _ = other;
        unimplemented!()
    }

    fn cross(self, other: Self) -> Self {
        _ = other;
        unimplemented!()
    }
}
