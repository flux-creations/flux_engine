trait VectorSpace<T: Num, S: Num> {
    fn new<T>(x: T, y: T) -> Self;

    fn zero<T>() -> Self;
    fn one<T>() -> Self;
    fn up<T>() -> Self;
    fn right<T>() -> Self;

    fn parse<S>(self: Self<T>) -> Option<Self<S>>;

    fn magnitude<T>(self: Self<T>) -> T;

    fn invert<T>(self: Self<T>) -> Self<T>;

    fn add<T>(self: Self<T>, other: Self<T>) -> Self<T>;
    fn dot<T>(self: Self<T>, other: Self<T>) -> Self<T>;
    fn cross<T>(self: Self<T>, other: Self<T>) -> Self<T>;
}

struct Vec2<T: Num> {
    x: T,
    y: T,
}

struct Vec3<T: Num> {
    x: T,
    y: T,
    z: T
}

impl VectorSpace for Vec3<T: Num, S: Num> {
    fn new<T>(x: T, y: T, z: T) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }
}

impl VectorSpace for Vec2<T: Num, S: Num> {
    fn new<T>(x: T, y: T) -> Vec2 {
        Vec2 {
            x: x,
            y: y
        }
    }

    fn zero<T>() -> Vec2 {
        Vec2::new<T>(T::zero(), T::zero())
    }

    fn one<T>() -> Vec2 {
        Vec2::new<T>(T::one(), T::one())
    }

    fn up<T>() -> Vec2 {
        Vec2::new<T>(T::zero(), T::one())
    }
    
    fn right<T>() -> Vec2 {
        Vec2::new<T>(T::one(), T::zero())
    }

    fn parse<S>(self: Vec2<T>) -> Option<Vec2<S>> {
        Vec2::new<S>(self.x as S, self.y as S)
    }

    fn invert<T>(self: Vec2<T>) -> Vec2<T> {
        Vec2::new<T>(-self.x, -self.y)
    }

    fn add<T>(self: Vec2<T>, other: Vec2<T>) -> Vec2<T> {
        Vec2::new<T>(self.x + other.x, self.y + other.y);
    }

    fn dot<T>(self: Vec2<T>, other: Vec2<T>) -> Vec2<T> {
        self.x * other.x + self.y * other.y
    }

    fn cross<T>(self: Vec2<T>, other: Vec2<T>) -> Vec3<T> {
        Vec3<T>::new(0, 0, self.x * other.y - (self.y * other.x))
    }
}