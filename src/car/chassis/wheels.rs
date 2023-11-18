pub struct WheelPair {
    pub radius: f64, // meters
    pub wheel_rpm: f64,
    pub is_drive_wheel: bool,
}

impl WheelPair {
    pub fn new(radius: f64, is_drive_wheel: bool) -> Self {
        Self {
            wheel_rpm: 0.0,
            radius,
            is_drive_wheel
        }
    }

    pub fn get_force(&self, torque: f64, load: f64) -> f64 {
        torque / self.radius // N
    }

    fn get_slip(&self, car_velocity: (f64, f64)) -> f64 {
        const RPM_TO_RADS: f64 = 0.10472;

        let (vel1, _) = car_velocity;
        let ang_vel: f64 = self.wheel_rpm * RPM_TO_RADS;

        (ang_vel * self.radius - vel1) / vel1
    }
}


trait Brakes {
    // who needs these?
}

impl Brakes for WheelPair {

}