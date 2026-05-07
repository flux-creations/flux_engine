use super::scalar::Scalar;

pub trait VectorSpace<T: Scalar>
where
    Self: Sized,
{
    type CrossOutput; // Associated type i implement this for each struct impl
    type CastType;

    fn zero() -> Self;
    fn one() -> Self;
    fn up() -> Self;
    fn right() -> Self;

    fn parse(self) -> Option<Self::CastType>;

    fn magnitude(self) -> T;

    fn invert(self) -> Self;

    fn add(self, other: Self) -> Self;
    fn dot(self, other: Self) -> T;
    fn cross(self, other: Self) -> Self::CrossOutput;
}
