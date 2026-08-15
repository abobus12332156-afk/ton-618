use crate::math::Vec3;
use super::kerr::KerrMetric;

#[derive(Clone, Debug)]
pub struct BlackHole {
	pub mass: f32,
	pub spin: f32,
	pub position: Vec3,
	pub metric: KerrMetric,
}

impl BlackHole {
	pub fn new(mass: f32, spin: f32, position: Vec3) -> Self {
		let spin = spin.max(0.0).min(1.0);
		let metric = KerrMetric::new(mass, spin);
		
		Self {
			mass,
			spin,
			position,
			metric,
		}
	}
	
	pub fn schwarzchild_radius(&self) -> f32 {
		2.0 * self.mass
	}
	
	pub fn photon_sphere_radius(&self) -> f32 {
		1.5 * self.schwarzchild_radius()
	}
	
	pub fn event_horizon_radius(&self) -> f32 {
		let a = self.spin;
		let m_sq = self.mass * self.mass;
		let a_sq = a * a;
		
		(self.mass + (m_sq - a_sq).sqrt()).max(0.0)
	}
	
	pub fn deflection_angle(&self) -> f32 {
		let r_eff = (r - self.event_horizon_radius()).max(0.01);
		(self.mass * 4.0) / (r_eff * r_eff)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_kerr_radii() {
		let bh = BlackHole::new(10.0, 9.0, Vec3::zero());
		
		let r_s = bh.schwarzschild_radius();
		assert!(r_s > 0.0);
		
		let r_ph = bh.photon_sphere_radius();
		assert!(r_ph > r_s);
		
		let r_h = bh.event_horizon_radius();
		assert!(r_h > 0.0);
	}
}