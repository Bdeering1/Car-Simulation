use std::fmt::Display as Display;
use std::f32::consts::PI as PI;

pub struct Car {
    pub engine: Engine,
    pub transmission: Transmission,
    pub wheels: Wheels,
    pub velocity: (f32, f32), // top speed should be 320 km/h
    pub acceleration: (f32, f32),
    pub mass: f32, // 1587.12 kg
    pub drag_coefficient: f32, // 0.34
    pub rr_coefficient: f32,
    pub drive_force: f32,
    pub drag: f32,
    pub rolling_res: f32,
}

impl Car {
    pub fn new(engine: Engine, transmission: Transmission, wheels: Wheels) -> Self {
        Self {
            engine,
            transmission,
            wheels,
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            mass: 1587.12,
            drag_coefficient: 0.34,
            rr_coefficient: 10.2,
            drive_force: 0.0,
            drag: 0.0,
            rolling_res: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.engine.rpm == self.engine.max_rpm {
            self.drive_force = 0.0
        } else {
            self.drive_force = self.wheels.get_force(self.engine.get_torque(1.0) * self.transmission.get_ratio());
        }
        self.drag = self.velocity.0.powf(2.0) * self.drag_coefficient;
        self.rolling_res = self.velocity.0 * self.rr_coefficient;
        self.drive_force -= self.drag + self.rolling_res;
        self.acceleration.0 = self.drive_force / self.mass; // m/s^2

        self.velocity.0 += self.acceleration.0 * dt; // m/s
        self.engine.rpm = ((self.velocity.0 / (2.0 * PI * self.wheels.radius) // wheel rev/s
                            * self.transmission.get_ratio() * 60.0) as u32) // engine rpm
                            .clamp(self.engine.idle_rpm, self.engine.max_rpm);
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        f.write_str(format!("vel: {:.2} km/h acc: {:.2} m/s^2 drag: {:.2} N rolling res: {:.2} N {}", self.velocity.0 * 3.6, self.acceleration.0, self.drag, self.rolling_res, self.engine).as_str())?;
        Ok(())
    }
}

pub struct Engine {
    pub torque_curve: (f32, f32, f32, f32),
    pub idle_rpm: u32,
    pub max_rpm: u32,
    pub rpm_range: u32,
    pub rpm: u32,
    pub torque: f32,
}

impl Engine {
    pub fn new(idle_rpm: u32, max_rpm: u32, torque_curve: (f32, f32, f32, f32)) -> Self {
        Self{
            torque_curve,
            idle_rpm,
            max_rpm,
            rpm_range: max_rpm - idle_rpm,
            rpm: idle_rpm,
            torque: 0.0,
        }
    }

    // t = (x-1000) / 7000
    // y = (v1)(1-t)^3 + (v2)(3t)(1-t)^2 + (v3)(3t^2)(1-t) + (v4)(t^3)
    // v1 = 200, v2 = 274, v3 = 380, v4 = 225
    pub fn get_torque(&mut self, throttle: f32) -> f32 {
        const FTLB_PER_NM: f32 = 1.35582;

        let t: f32 = (self.rpm - self.idle_rpm) as f32 / self.rpm_range as f32;
        let (v1, v2, v3, v4) = self.torque_curve;
        self.torque = ((v1 * (1.0-t).powf(3.0)) + (v2 * (3.0*t) * (1.0-t).powf(2.0)) + (v3 * 3.0 * t.powf(2.0) * (1.0-t)) + (v4 * t.powf(3.0))) * throttle * FTLB_PER_NM;
        self.torque // Nm
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("rpm: {} torque: {} Nm", self.rpm, self.torque).as_str())?;
        Ok(())
    }
}

pub struct Transmission {
    gear_ratios: Vec<f32>,
    diff_ratio: f32,
    pub gear: u8,
}

impl Transmission {
    pub fn new(diff_ratio: f32, gear_ratios: Vec<f32>) -> Self {
        Self {
            gear_ratios,
            diff_ratio,
            gear: 1
        }
    }

    pub fn get_ratio(&self) -> f32 {
        (*self.gear_ratios.get(self.gear as usize).expect("Error: invalid gear")) * self.diff_ratio
    }
}


pub struct Wheels {
    pub wheel_rpm: f32,
    pub radius: f32, // meters
}

impl Wheels {
    pub fn new(radius: f32) -> Self {
        Self {
            wheel_rpm: 0.0,
            radius
        }
    }

    pub fn get_force(&self, torque: f32) -> f32 {
        torque / self.radius // N
    }
}


trait Brakes {
    // who needs these?
}