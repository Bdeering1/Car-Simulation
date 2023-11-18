use std::fmt::Display;

pub struct Engine {
    pub torque_curve: fn(u32) -> f64,
    pub idle_rpm: u32,
    pub max_rpm: u32,
    pub rpm_range: u32,
    pub rpm: u32,
    pub torque: f64,
}

impl Engine {
    pub fn new(idle_rpm: u32, max_rpm: u32, torque_curve: fn(u32) -> f64) -> Self {
        Self{
            torque_curve,
            idle_rpm,
            max_rpm,
            rpm_range: max_rpm - idle_rpm,
            rpm: idle_rpm,
            torque: 0.0,
        }
    }

    pub fn get_torque(&mut self, throttle: f64) -> f64 {
        const LBFT_PER_NM: f64 = 1.35582;

        self.torque = (self.torque_curve)(self.rpm) * throttle * LBFT_PER_NM;
        self.torque // Nm
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("rpm: {} torque: {} Nm", self.rpm, self.torque as i32).as_str())?;
        Ok(())
    }
}
