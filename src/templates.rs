use crate::car::{Car, Engine, Transmission, Wheels};

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
            // https://www.conceptcarz.com/s28169/audi-r8.aspx
            // http://www.goapr.com.mt/products/ecu_upgrade_52fsi_r8.html
            // https://www.caranddriver.com/audi/r8/specs
            CarType::AudiR8 => {
                let engine = Engine::new(1000, 8400, (240.0, 399.0, 460.0, 322.0));
                let transmission = Transmission::new(3.59 ,vec![-2.65, 0.0, 3.133, 2.588, 1.880, 1.140, 0.898, 0.884, 0.653]);
                let wheels = Wheels::new(0.254);
                Car::new(engine, transmission, wheels, 1587.12, 0.34, 10.2) 
            },
            // https://www.motortrend.com/reviews/2022-tesla-model-s-plaid-first-test-review/
            // https://www.tesla.com/sites/default/files/blog_attachments/the-slipperiest-car-on-the-road.pdf
            CarType::TeslaSPlaid => {
                let engine = Engine::new(1000, 23000, (1050.0, 1050.0, 250.0, 250.0));
                let transmission = Transmission::new(7.5 ,vec![-1.0, 0.0, 1.0]);
                let wheels = Wheels::new(0.2667);
                Car::new(engine, transmission, wheels, 2184.501, 0.24, 10.2)
            }
        }
    } 
}