use std::fmt::Display as Display;

use libm::powf;

pub struct WheelPair {
    pub is_drive_wheel: bool,
    pub mass: f64,
    pub radius: f64, // meters
    pub ang_vel: f64, //rads/s
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
            inertia: (mass * powf(radius as f32, 2.0) as f64) / 2.0, //inertia of a cylinder = mass * radius^2 / 2
            traction: 0.0,
            slip_ratio: 0.0,
        }
    }

    pub fn get_force(&mut self, torque: f64, load: f64, car_velocity: (f64, f64), dt: f64) -> f64 {
        self.slip_ratio = self.get_slip(car_velocity);
        println!("{}", self.slip_ratio);
        self.traction = self.get_traction(load, self.slip_ratio).abs();

        let traction_torque = self.traction * self.radius;
        self.calc_wheel_acceleration(traction_torque, torque, 0.0 /* TODO: add brakes */, dt);

        if self.is_drive_wheel {
            self.traction
        } else {
            0.0
        }
    }

    fn get_slip(&self, car_velocity: (f64, f64)) -> f64 {
        let (vel1, _) = car_velocity;

        if vel1 == 0.0 {0.0}
        else {
            ((self.ang_vel * self.radius) - vel1) / vel1.abs()
        }
    }

    fn calc_wheel_acceleration(&mut self, traction_torque: f64, torque: f64, brake_torque: f64, dt: f64) {
        let total_torque = torque - traction_torque - brake_torque;
        let wheel_acceleration = total_torque / self.inertia;
        self.ang_vel += wheel_acceleration * dt;
        //println!("{}", torque);
    }

    fn get_traction(&self, load: f64, mut slip_ratio: f64) -> f64 { //traction in longitudinal force (N)
        slip_ratio = slip_ratio.clamp(0.0, 70.0);
        let traction_force: f64 = f64::min(200.0 * slip_ratio, 1200.0);
        
        traction_force * (load/1000.0)
    }
}

impl Display for WheelPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("angular vel: {} slip ratio: {} traction force: {}", self.ang_vel as i32, self.slip_ratio as i32, self.traction as i32).as_str())?;
        Ok(())
    }
}

trait Brakes {

}

impl Brakes for WheelPair {

}