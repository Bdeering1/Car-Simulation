use std::fmt::Display as Display;
use std::f64::consts::PI as PI;

pub struct Car {
    pub engine: Engine,
    pub transmission: Transmission,
    pub wheels: Wheels,
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub mass: f64,
    pub drag_coefficient: f64,
    pub rr_coefficient: f64,
    pub drive_force: f64,
    pub drag: f64,
    pub rolling_res: f64,
    pub hp: f64,
}

impl Car {
    pub fn new(engine: Engine, transmission: Transmission, wheels: Wheels, mass: f64, drag_coefficient: f64, rr_coefficient: f64) -> Self {
        Self {
            engine,
            transmission,
            wheels,
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            mass,
            drag_coefficient,
            rr_coefficient,
            drive_force: 0.0,
            drag: 0.0,
            rolling_res: 0.0,
            hp: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        const DRIVE_TRAIN_EFFICIENCY: f64 = 0.75;

        if self.engine.rpm > 8000 && self.transmission.gear < self.transmission.max_gear { self.transmission.gear += 1; }

        if self.engine.rpm == self.engine.max_rpm {
            self.drive_force = 0.0
        } else {
            self.drive_force = self.wheels.get_force(self.engine.get_torque(1.0) * self.transmission.get_ratio());
        }
        self.drive_force *= DRIVE_TRAIN_EFFICIENCY;
        self.drag = self.velocity.0.powf(2.0) * self.drag_coefficient;
        self.rolling_res = self.velocity.0 * self.rr_coefficient;
        self.drive_force -= self.drag + self.rolling_res;

        self.acceleration.0 = self.drive_force / self.mass; // m/s^2
        self.velocity.0 += self.acceleration.0 * dt; // m/s

        self.engine.rpm = ((self.velocity.0 / (2.0 * PI * self.wheels.radius) // wheel rev/s
                            * self.transmission.get_ratio() * 60.0) as u32) // engine rpm
                            .clamp(self.engine.idle_rpm, self.engine.max_rpm);

        self.hp = (self.engine.get_torque(1.0) / 1.35582) * self.engine.rpm as f64 / 5252.0;
    }
}


impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        f.write_str(format!("vel: {:.2} km/h acc: {:.2} m/s^2 drive force: {} N hp: {} drag: {} N rolling res: {} N {} {}", self.velocity.0 * 3.6, self.acceleration.0, self.drive_force as i32, self.hp as i32, self.drag as i32, self.rolling_res as i32, self.engine, self.transmission).as_str())?;
        Ok(())
    }
}

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

pub struct Transmission {
    gear_ratios: Vec<f64>,
    diff_ratio: f64,
    pub gear: u8,
    pub max_gear: u8,
}

impl Transmission {
    pub fn new(diff_ratio: f64, gear_ratios: Vec<f64>) -> Self {
        Self {
            max_gear: (gear_ratios.len() - 1) as u8,
            gear_ratios,
            diff_ratio,
            gear: 1,
        }
    }

    pub fn get_ratio(&self) -> f64 {
        (*self.gear_ratios.get(self.gear as usize).expect("Error: invalid gear")) * self.diff_ratio
    }
}

impl Display for Transmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("gear: {}", self.gear - 1).as_str())?;
        Ok(())
    }
}


pub struct Wheels {
    pub wheel_rpm: f64,
    pub radius: f64, // meters
}

impl Wheels {
    pub fn new(radius: f64) -> Self {
        Self {
            wheel_rpm: 0.0,
            radius
        }
    }

    pub fn get_force(&self, torque: f64) -> f64 {
        torque / self.radius // N
    }
}


trait Brakes {
    // who needs these?
}