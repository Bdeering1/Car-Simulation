use std::fmt::Display as Display;
use std::f64::consts::PI as PI;

pub struct Car {
    pub engine: Engine,
    pub transmission: Transmission,
    pub chassis: Chassis,
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
    pub fn new(engine: Engine, transmission: Transmission, chassis: Chassis, drag_coefficient: f64, rr_coefficient: f64) -> Self {
        Self {
            mass: chassis.static_load.0 + chassis.static_load.1,
            engine,
            transmission,
            chassis,
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
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
            self.drive_force = self.chassis.get_wheel_force(self.drive_force, self.engine.get_torque(1.0) * self.transmission.get_ratio()).0;
        }
        self.drive_force *= DRIVE_TRAIN_EFFICIENCY;
        self.drag = self.velocity.0.powf(2.0) * self.drag_coefficient;
        self.rolling_res = self.velocity.0 * self.rr_coefficient;
        self.drive_force -= self.drag + self.rolling_res;

        self.acceleration.0 = self.drive_force / self.mass; // m/s^2
        self.velocity.0 += self.acceleration.0 * dt; // m/s

        self.engine.rpm = ((self.velocity.0 / (2.0 * PI * self.chassis.front_wheels.radius) // wheel rev/s
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



#[derive(PartialEq)]
pub enum DriveWheels {
    Front, 
    Rear,
    All
}

pub struct Chassis {
    pub front_wheels: WheelPair,
    pub rear_wheels: WheelPair,
    wheel_base: f64,
    cg_height: f64,
    static_load: (f64, f64), // front, rear
    torque_dist: (f64, f64)
}

impl Chassis {
    pub fn new(wheel_radius: f64, wheel_base: f64, cg_height: f64, weight_ratio: f64, weight: f64, drive_wheels: DriveWheels) -> Self {
        let front_wheels = WheelPair::new(wheel_radius, drive_wheels == DriveWheels::Front || drive_wheels == DriveWheels::All);
        let rear_wheels = WheelPair::new(wheel_radius, drive_wheels == DriveWheels::Rear || drive_wheels == DriveWheels::All);
        Chassis {
            front_wheels,
            rear_wheels,
            wheel_base,
            cg_height,
            static_load: (weight * weight_ratio, weight * (1.0 - weight_ratio)),
            torque_dist: (match drive_wheels { DriveWheels::Front => 1.0, DriveWheels::All => 0.5, _ => 0.0 },
                          match drive_wheels { DriveWheels::Rear => 1.0, DriveWheels::All => 0.5, _ => 0.0 },)
        }
    }

    pub fn get_wheel_force(&self, drive_force: f64, torque: f64) -> (f64, f64) {
        let (front_load, rear_load) = self.distribute_weight(drive_force);
        let (front_force, rear_force) = (self.front_wheels.get_force(torque * self.torque_dist.0, front_load), 
                                                   self.rear_wheels.get_force(torque * self.torque_dist.1, rear_load));

        (front_force + rear_force, 0.0)
    }

    fn distribute_weight(&self, drive_force: f64) -> (f64, f64) {
        let shift_to_rear = (self.cg_height / self.wheel_base) * drive_force;
        
        (self.static_load.0 - shift_to_rear, self.static_load.1 + shift_to_rear)
    }

}

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