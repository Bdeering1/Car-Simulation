use crate::car::{Car, Engine, Transmission, Wheels};

pub enum CarType {
    AudiR8,
}

pub trait Templates {
    fn from_template(template: CarType) -> Self;
}

impl Templates for Car {
    fn from_template(template: CarType) -> Self {
        match template {
            CarType::AudiR8 => {
                let engine = Engine::new(1000, 8400, (240.0, 399.0, 460.0, 322.0));
                let transmission = Transmission::new(3.59 ,vec![-2.65, 0.0, 3.133, 2.588, 1.880, 1.140, 0.898, 0.884, 0.653]);
                let wheels = Wheels::new(0.254);
                Car::new(engine, transmission, wheels, 1587.12, 0.34, 10.2) 
            }
        }
    } 
}