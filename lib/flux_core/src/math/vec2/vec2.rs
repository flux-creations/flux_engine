use num::{Num};

pub trait VectorSpace<T: Num, S: Num> where Self: Sized {
    fn zero() -> Self;
    fn one() -> Self;
    fn up() -> Self;
    fn right() -> Self;

    fn parse(self: Self) -> Option<Self>;

    fn magnitude(self: Self) -> T;

    fn invert(self: Self) -> Self;

    fn add(self: Self, other: Self) -> Self;
    fn dot(self: Self, other: Self) -> T;
    fn cross(self: Self, other: Self) -> Self;
}

pub struct Vec2<T: Num> {
    x: T,
    y: T,
}

pub struct Vec3<T: Num> {
    x: T,
    y: T,
    z: T
}

impl<T: Num> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
}

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x: x,
            y: y
        }
    }
}

impl<T: Num, S: Num> VectorSpace<T, S> for Vec3<T> {
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

    fn parse(self: Self) -> Option<Self> {
        _ = self.x + self.y + self.z;
        unimplemented!()
    }

    fn magnitude(self: Self) -> T {
        unimplemented!()
    }

    fn invert(self: Self) -> Self {
        unimplemented!()
    }

    fn add(self: Self, other: Self) -> Self {
        _ = other;
        unimplemented!()
    }

    fn dot(self: Self, other: Self) -> T {
        _ = other;
        unimplemented!()
    }

    fn cross(self: Self, other: Self) -> Self {
        _ = other;
        unimplemented!()
    }
}

impl<T: Num, S: Num> VectorSpace<T, S> for Vec2<T> {
    fn magnitude(self: Self) -> T {
        unimplemented!()
    }

    fn zero() -> Vec2<T> {
        Vec2::new(T::zero(), T::zero())
    }

    fn one() -> Vec2<T> {
        Vec2::new(T::one(), T::one())
    }

    fn up() -> Vec2<T> {
        Vec2::new(T::zero(), T::one())
    }
    
    fn right() -> Vec2<T> {
        Vec2::new(T::one(), T::zero())
    }

    // TODO: Find a way to seamlessly parse between number over Num trait
    fn parse(self: Vec2<T>) -> Option<Vec2<T>> {
        unimplemented!()
    }

    //TODO: Not all Num trait types are having simple way to invert find best solution
    fn invert(self: Vec2<T>) -> Vec2<T> {
        unimplemented!()
    }

    fn add(self: Vec2<T>, other: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    fn dot(self: Vec2<T>, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
    // TODO: Why this shows Vec2??
    fn cross(self: Vec2<T>, other: Vec2<T>) -> Vec2<T> {
        _ = other;
        //Vec3::new(T::zero(), T::zero(), (self.x * other.y - (self.y * other.x) as T))
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_new() {
        let vec2: Vec2<i32> = Vec2::new(3, 5);

        assert_eq!(vec2.x, 3);
        assert_eq!(vec2.y, 5);
    }
}