mod wheels;

use wheels::WheelPair;

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
    pub static_load: (f64, f64), // front, rear
    torque_dist: (f64, f64)
}

impl Chassis {
    pub fn new(wheel_radius: f64, wheel_base: f64, height: f64, weight_ratio: f64, weight: f64, drive_wheels: DriveWheels) -> Self {
        const CG_HEIGHT_RATIO: f64 = 0.45; 

        let front_wheels = WheelPair::new(wheel_radius, drive_wheels == DriveWheels::Front || drive_wheels == DriveWheels::All);
        let rear_wheels = WheelPair::new(wheel_radius, drive_wheels == DriveWheels::Rear || drive_wheels == DriveWheels::All);
        Chassis {
            front_wheels,
            rear_wheels,
            wheel_base,
            cg_height: height * CG_HEIGHT_RATIO,
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
