use libm::powf;

pub struct WheelPair {
    pub radius: f64, // meters
    pub ang_vel: f64, //rads/s
    pub mass: f64,
    inertia: f64,
    pub is_drive_wheel: bool,
}

impl WheelPair {
    pub fn new(radius: f64, mass: f64, is_drive_wheel: bool) -> Self {
        Self {
            ang_vel: 0.0,
            radius,
            mass,
            inertia: (mass * powf(radius as f32, 2.0) as f64) / 2.0, //inertia of a cylinder = mass * radius^2 / 2
            is_drive_wheel
        }
    }

    pub fn get_force(&mut self, torque: f64, load: f64, car_velocity: (f64, f64), dt: f64) -> f64 {
        let slip = self.get_slip(car_velocity);
        let traction: f64 = self.get_traction(load, slip);

        self.calc_wheel_acceleration(traction, torque, 0.0 /* TODO: add brakes */, dt);

        traction
    }

    fn get_slip(&self, car_velocity: (f64, f64)) -> f64 {
        let (vel1, _) = car_velocity;

        (self.ang_vel * self.radius - vel1) / vel1
    }

    fn calc_wheel_acceleration(&mut self, traction: f64, torque: f64, brake_torque: f64, dt: f64) {
        let total_torque = traction + torque + brake_torque;

        let wheel_acceleration = total_torque / self.inertia;
        self.ang_vel += wheel_acceleration * dt;
    }


    //because the graph is scaled to be from 0-1:
    //force value: load_in_newtons*1.2
    //slip ratio value: 20 
    //curve calculation i got from playing around in desmos:
    //7.19577(x)^{0.655221} + (-27.9498)(x+0.961159)^{0.290593}+27.6299
    fn get_traction(&self, load: f64, slip_ratio: f64) -> f64 { //traction in longitudinal force (N)
        let slip_ratio: f64 = slip_ratio * 20.0;
        let traction_force = f64::min(1.0,
             (powf((7.19577 * slip_ratio) as f32, 0.655221) - 27.9498 * powf((slip_ratio + 0.961159) as f32, 0.290593) + 27.6299) as f64);
        
        
        traction_force * load * 1.2
    }
}


trait Brakes {
    // who needs these?
}

impl Brakes for WheelPair {

}