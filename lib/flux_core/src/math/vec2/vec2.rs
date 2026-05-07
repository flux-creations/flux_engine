use super::scalar::Scalar;
use super::vec3::Vec3;
use super::vector::VectorSpace;

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

impl<T: Scalar> VectorSpace<T> for Vec2<T> {
    type CrossOutput = Vec3<T>;
    type CastType = T;

    fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
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
    fn parse(self: Vec2<T>) -> Option<T> {
        unimplemented!()
    }

    //TODO: Not all Num trait types are having simple way to invert find best solution
    fn invert(self: Vec2<T>) -> Vec2<T> {
        // Vec2::new(self.x * (-1 as Self::CastType), self.y * (-1))
        unimplemented!()
    }

    fn add(self: Vec2<T>, other: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    fn dot(self: Vec2<T>, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    fn cross(self: Vec2<T>, other: Vec2<T>) -> Vec3<T> {
        Vec3::new(T::zero(), T::zero(), self.x * other.y - (self.y * other.x))
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

    #[test]
    fn test_vec2_addition() {
        let vec2: Vec2<i32> = Vec2::new(3, 5);
        let vec2_other: Vec2<i32> = Vec2::new(3, 0);

        let vec2_addition_result: Vec2<i32> = vec2.add(vec2_other);

        assert_eq!(vec2_addition_result.x, 6);
        assert_eq!(vec2_addition_result.y, 5);
    }

    #[test]
    fn test_vec2_magnitude() {
        let vec2: Vec2<i32> = Vec2::new(2, 2);

        let vec2_magnitude = vec2.magnitude();

        assert_eq!(vec2_magnitude, 3_i32);
    }

    #[test]
    fn test_vec2_dot() {
        let vec2: Vec2<i32> = Vec2::new(2, 2);
        let vec2_other: Vec2<i32> = Vec2::new(2, 2);

        let vec2_dot_product_result: i32 = vec2.dot(vec2_other);

        assert_eq!(vec2_dot_product_result, 8);
    }

    #[test]
    fn test_vec2_cross() {
        let vec2: Vec2<i32> = Vec2::new(2, 2);
        let vec2_other: Vec2<i32> = Vec2::new(2, 2);

        let vec2_cross_product_result: Vec3<i32> = vec2.cross(vec2_other);
        _ = vec2_cross_product_result;
    }
}
