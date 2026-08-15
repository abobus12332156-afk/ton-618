pub mod blackhole;
pub mod kerr;

use glam::Vec3;
pub use blackhole::BlackHole;
pub use kerr::KerrMetric;

use crate::math::Vec3;

pub struct BlackHoleParams {
	pub mass: f32,
	pub spin: f32,
	pub position: Vec3,
}

impl Default for BlackHoleParams {
	fn default() -> Self {
		Self {
			mass: 10.0,
			spin: 0.9,
			position: Vec3::zero();
		}
	}	
}