mod car;

use std::{thread, time};
use car::Car;
use car::templates::{Templates, CarType};

fn main() {
    let mut car = Car::from_template(CarType::AudiR8);
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
