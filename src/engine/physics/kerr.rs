#[derive(Clone, Debug)]
pub struct KerrMetric {
    pub mass: f32,
    pub spin: f32,
}

impl KerrMetric {
    pub fn new(mass: f32, spin: f32) -> Self {
        Self {
            mass,
            spin: spin.max(0.0).min(1.0),
        }
    }

    pub fn boyer_lindquist(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
        let rho_sq = x * x + y * y + z * z;
        let r = rho_sq.sqrt();
        let theta = if r > 0.0 { (z / r).acos() } else { 0.0 };

        (r, theta)
    }

    pub fn frame_dragging_velocity(&self, r: f32, theta: f32) -> f32 {
        if r < 0.1 {
            return 0.0;
        }

        let a = self.spin * self.mass;
        let sin_theta = theta.sin();

        (2.0 * a * self.mass) / (r * r * r + a * a * r)
    }
}
