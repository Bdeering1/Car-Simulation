mod engine;
mod transmission;
mod chassis;
pub mod templates;

use std::fmt::Display as Display;
use std::f64::consts::PI as PI;
use engine::Engine;
use transmission::Transmission;
use chassis::Chassis;

use crate::car::chassis::DriveWheels;

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
            self.drive_force = self.chassis.get_wheel_force(self.drive_force, self.engine.get_torque(1.0) * self.transmission.get_ratio(), self.velocity, dt).0;
        }
        self.drive_force *= DRIVE_TRAIN_EFFICIENCY;
        self.drag = self.velocity.0.powf(2.0) * self.drag_coefficient;
        self.rolling_res = self.velocity.0 * self.rr_coefficient;
        self.drive_force -= self.drag + self.rolling_res;

        self.acceleration.0 = self.drive_force / self.mass; // m/s^2
        self.velocity.0 += self.acceleration.0 * dt; // m/s

        let drive_wheel_rpm: f64 = match self.chassis.drive_wheels {
            DriveWheels::Front => self.chassis.front_wheels.ang_vel * 9.549296585513721,
            DriveWheels::Rear => self.chassis.rear_wheels.ang_vel * 9.549296585513721,
            DriveWheels::All => ((self.chassis.front_wheels.ang_vel + self.chassis.rear_wheels.ang_vel) * 9.549296585513721)/2.0, //avg velocity between both sets of wheels
        };

        self.engine.rpm = ((drive_wheel_rpm /* wheel rev/s */ 
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