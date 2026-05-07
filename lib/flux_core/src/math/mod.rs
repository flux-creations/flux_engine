// I geniuenly would want to commit violence on a guy who tought this is
// the right thing to make modules
#[path = "vec2/vec2.rs"]
mod vec2;

#[path = "vector/vector.rs"]
mod vector;

#[path = "vec3/vec3.rs"]
mod vec3;

#[path = "scalar/scalar.rs"]
mod scalar;

pub use scalar::*;
pub use vec2::*;
pub use vec3::*;
pub use vector::*;
