use std::fmt::Display as Display;

struct Car {
    engine: Engine,
    transmission: Transmission,
    wheels: Wheels,
    velocity: (f32, f32),
    acceleration: (f32, f32),
    mass: f32,
    drag_coefficient: f32
}

impl Car {

}

pub struct Engine {
    torque_curve: (f32, f32, f32, f32),
    idle_rpm: u32,
    rpm_range: u32,
    rpm: u32,
}

impl Engine {
    // t = (x-1000) / 7000
    // y = (v1)(1-t)^3 + (v2)(3t)(1-t)^2 + (v3)(3t^2)(1-t) + (v4)(t^3)
    // v1 = 200, v2 = 274, v3 = 380, v4 = 225
    pub fn get_torque(&self) -> f32 {
        let t: f32 = (self.rpm - self.idle_rpm) as f32 / self.rpm_range as f32;
        let (v1, v2, v3, v4) = self.torque_curve;
        (v1 * (1.0-t).powf(3.0)) + (v2 * (3.0*t) * (1f32-t).powf(2.0)) + (v3 * (3.0*t).powf(2.0) * (1.0-t)) + (v4 * t.powf(3.0))
    }

    pub fn set_rpm(&mut self, rpm: u32) {
        self.rpm = rpm;
    }

    pub fn new(idle_rpm: u32, max_rpm: u32, torque_curve: (f32, f32, f32, f32)) -> Self {
        Self{
            torque_curve,
            idle_rpm,
            rpm_range: max_rpm - idle_rpm,
            rpm: idle_rpm
        }
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("rpm: {}", self.rpm).as_str())?;
        Ok(())
    }
}

struct Transmission {

}

impl Transmission {

}


struct Wheels {

}

impl Wheels {

}


trait Brakes {

}