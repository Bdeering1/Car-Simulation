use std::fmt::Display as Display;

use libm::powf;

pub struct WheelPair {
    pub is_drive_wheel: bool,
    pub mass: f64,
    pub radius: f64, // meters
    pub ang_vel: f64, // rads/s
    pub inertia: f64,
    pub traction: f64,
    pub slip_ratio: f64
}

impl WheelPair {
    pub fn new(radius: f64, mass: f64, is_drive_wheel: bool) -> Self {
        Self {
            is_drive_wheel,
            mass,
            radius,
            ang_vel: 0.0,
            inertia: (mass * powf(radius as f32, 2.0) as f64) / 2.0, // inertia of a cylinder = mass * radius^2 / 2
            traction: 0.0,
            slip_ratio: 0.0,
        }
    }

    pub fn get_force(&mut self, torque: f64, down_force: f64, car_velocity: (f64, f64), dt: f64) -> f64 {
        self.slip_ratio = self.get_slip(car_velocity);
        //println!("{}", self.slip_ratio);
        self.traction = self.get_traction(down_force, self.slip_ratio);

        let traction_torque = self.traction * self.radius;
        self.calc_wheel_acceleration(traction_torque, torque, 0.0 /* TODO: add brakes */, dt);

        if self.traction == 0.0 { torque / self.radius } else { f64::min(torque / self.radius, self.traction) }
    }

    fn get_slip(&self, car_velocity: (f64, f64)) -> f64 {
        let (vel1, _) = car_velocity;

        if vel1 == 0.0 { 0.0 } else { ((self.ang_vel * self.radius) - vel1) / vel1.abs() }
    }

    fn calc_wheel_acceleration(&mut self, traction_torque: f64, torque: f64, brake_torque: f64, dt: f64) {
        let total_torque = torque - traction_torque - brake_torque;
        // ^ this might be wrong (frequently results in a negative wheel acceleration)
        // also if torque == traction torque (perfect traction) then total torque = 0 and therefore acceleration = 0
        //println!("total torque: {} torque: {} traction torque: {}", total_torque, torque, traction_torque);
        let wheel_acceleration = total_torque / self.inertia;
        self.ang_vel += wheel_acceleration * dt;
    }

    fn get_traction(&self, down_force: f64, slip_ratio: f64) -> f64 { // traction in longitudinal force (N)
        let traction_force: f64 = (200.0 * slip_ratio).clamp(-1200.0, 1200.0);
        // ^ this shouldn't be a "force" value because otherwise the units N*N -> N^2
        
        traction_force * down_force / 1000.0
    }
}

impl Display for WheelPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("angular vel: {} slip ratio: {} traction: {} N", self.ang_vel as i32, self.slip_ratio as i32, self.traction as i32).as_str())?;
        Ok(())
    }
}

trait Brakes {

}

impl Brakes for WheelPair {

}