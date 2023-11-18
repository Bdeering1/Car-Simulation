use libm::tanh;
use crate::car::{Car, Engine, Transmission, Chassis};
use crate::car::chassis::DriveWheels;

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
            // https://www.autoblog.com/2009/08/31/first-drive-2010-audi-r8-v10-answers-our-power-prayers/
            // http://www.goapr.com.mt/products/ecu_upgrade_52fsi_r8.html
            CarType::AudiR8 => {
                let torque_curve = |rpm| -4371.57 * ((1.0 / 12600.0) * rpm as f64 - 1.0).powf(3.0) - 0.0000588218 * (rpm as f64 - 1.0).powf(2.0) + rpm as f64 - 4082.43;
                let engine = Engine::new(1000, 8700, torque_curve);
                let transmission = Transmission::new(3.59 ,vec![-2.65, 0.0, 3.133, 2.588, 1.880, 1.140, 0.898, 0.884, 0.653]);
                let chassis = Chassis::new(0.254, 2.64922, 0.0 /* ?? */, 0.0 /*??*/, 0.44, 1587.12, DriveWheels::Front);
                Car::new(engine, transmission, chassis, 0.34, 10.2) 
            },
            // 0-60 mph: 2s, 0-100 mph: 4s, 0-200 mph: 16s, peak hp: 1020
            // https://www.motortrend.com/reviews/2022-tesla-model-s-plaid-first-test-review/
            // https://www.tesla.com/sites/default/files/blog_attachments/the-slipperiest-car-on-the-road.pdf
            // https://teslamotorsclub.com/tmc/threads/technical-deep-dive-plaid-torque-curve.240710/
            CarType::TeslaSPlaid => {
                let torque_curve = |rpm| -387.669 * tanh((1.0 / 5000.0) * rpm as f64 - 1.98572) + 683.534;
                let engine = Engine::new(1000, 23300, torque_curve);
                let transmission = Transmission::new(7.5 ,vec![-1.0, 0.0, 1.0]);
                let chassis = Chassis::new(0.2667, 2.9591, 0.0 /* ?? */, 0.0 /*??*/, 0.48, 2184.501, DriveWheels::All);
                Car::new(engine, transmission, chassis, 0.24, 10.2)
            }
        }
    } 
}