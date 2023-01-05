mod car;
use car::{Car, Engine, Transmission, Wheels};
use std::{thread, time};

fn main() {
    let engine = Engine::new(1000, 8400, (240.0, 399.0, 460.0, 322.0));
    let transmission = Transmission::new(3.59 ,vec![-2.65, 0.0, 3.133, 2.588, 1.880, 1.140, 0.898, 0.884, 0.653]);
    let wheels = Wheels::new(0.2032);
    let mut car = Car::new(engine, transmission, wheels);

    car.transmission.gear = 2;

    let mut time_passed: f32 = 0.0;
    loop {
        car.update(0.01);
        if (time_passed * 100.0) as i32 % 20 == 0 {
            println!("{} time elapsed: {:.1}s", car, time_passed);
            thread::sleep(time::Duration::from_millis(200));
        }
        time_passed += 0.01;
        time_passed = (time_passed * 100.0).round() / 100.0;
    }
}
