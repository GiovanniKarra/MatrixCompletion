
pub trait Float: num::Float + std::iter::Sum + std::ops::AddAssign {}

impl Float for f32 {}
impl Float for f64 {}


pub trait Dot<T: Float> {
	fn dot(&self, other: &Self) -> T;
}