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
        self.traction = self.get_traction(load, self.slip_ratio);

        if self.is_drive_wheel {
            self.calc_wheel_acceleration(self.traction, torque, 0.0 /* TODO: add brakes */, dt);
        }

        self.traction
    }

    fn get_slip(&self, car_velocity: (f64, f64)) -> f64 {
        let (vel1, _) = car_velocity;

        if vel1 == 0.0 {
            0.0
        } else {
            ((self.ang_vel * self.radius) - vel1) / vel1
        }
    }

    fn calc_wheel_acceleration(&mut self, traction: f64, torque: f64, brake_torque: f64, dt: f64) {
        let total_torque = torque - traction - brake_torque;

        let wheel_acceleration = total_torque / self.inertia;
        self.ang_vel += wheel_acceleration * dt;
    }


    //because the graph is scaled to be from 0-1:
    //force value: load_in_newtons*1.2
    //slip ratio value: 20 
    //curve calculation i got from playing around in desmos:
    //7.19577(x)^{0.655221} + (-27.9498)(x+0.961159)^{0.290593}+27.6299
    fn get_traction(&self, load: f64, slip_ratio: f64) -> f64 { //traction in longitudinal force (N)
        let slip: f64 = slip_ratio * 20.0;
        let traction_force = f64::min(1.0,
             (powf((7.19577 * slip) as f32, 0.655221) - 27.9498 * powf((slip + 0.961159) as f32, 0.290593) + 27.6299) as f64);
        
        traction_force * load * 1.2 * self.radius
    }
}

impl Display for WheelPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("angular vel: {} slip ratio: {} traction force: {}", self.ang_vel, self.slip_ratio, self.traction as i32).as_str())?;
        Ok(())
    }
}

trait Brakes {

}

impl Brakes for WheelPair {

}