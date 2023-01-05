use libm::tanh;
use crate::car::{Car, Engine, Transmission, Wheels};

#[allow(dead_code)]
pub enum CarType {
    AudiR8,
    TeslaSPlaid,    
}

pub trait Templates {
    fn from_template(template: CarType) -> Self;
}

impl Templates for Car {
    fn from_template(template: CarType) -> Self {
        match template {
            // 0-60 mph in 3.2-3.8s, top speed: 200 mph
            // https://www.conceptcarz.com/s28169/audi-r8.aspx
            // https://www.caranddriver.com/audi/r8/specs
            // http://www.goapr.com.mt/products/ecu_upgrade_52fsi_r8.html
            CarType::AudiR8 => {
                let torque_curve = |rpm| -4371.57 * ((1.0 / 12600.0) * rpm as f64 - 1.0).powf(3.0) - 0.0000588218 * (rpm as f64 - 1.0).powf(2.0) + rpm as f64 - 4082.43;
                let engine = Engine::new(1000, 8700, torque_curve);
                let transmission = Transmission::new(3.59 ,vec![-2.65, 0.0, 3.133, 2.588, 1.880, 1.140, 0.898, 0.884, 0.653]);
                let wheels = Wheels::new(0.254);
                Car::new(engine, transmission, wheels, 1587.12, 0.34, 10.2) 
            },
            // 0-60 mph: 2s, 0-100 mph: 4s, 0-200 mph: 16s, peak hp: 1020
            // https://www.motortrend.com/reviews/2022-tesla-model-s-plaid-first-test-review/
            // https://www.tesla.com/sites/default/files/blog_attachments/the-slipperiest-car-on-the-road.pdf
            CarType::TeslaSPlaid => {
                let torque_curve = |rpm| -387.669 * tanh((1.0 / 5000.0) * rpm as f64 - 1.98572) + 683.534;
                let engine = Engine::new(1000, 23300, torque_curve);
                let transmission = Transmission::new(7.5 ,vec![-1.0, 0.0, 1.0]);
                let wheels = Wheels::new(0.2667);
                Car::new(engine, transmission, wheels, 2184.501, 0.24, 10.2)
            }
        }
    } 
}