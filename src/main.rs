mod car;

use std::{thread, time};
use car::Car;
use car::templates::{Templates, CarType};

fn main() {
    let mut car = Car::from_template(CarType::AudiR8);
    car.transmission.gear = 2;

    const TICKS_PER_SECOND: f64 = 10_000.0;
    const DISPLAY_RATE: i32 = 100;  // number of ticks between each display update

    let dt_step: f64 = 1.0 / TICKS_PER_SECOND;
    let mut total_time: f64 = 0.0;
    loop {
        car.update(dt_step);
        if (total_time * TICKS_PER_SECOND) as i32 % DISPLAY_RATE == 0 {
            println!("{} time elapsed: {:.2}s", car, total_time);
            thread::sleep(time::Duration::from_millis(200));
        }
        total_time += dt_step;
        total_time = (total_time * TICKS_PER_SECOND).round() / TICKS_PER_SECOND;
    }
}
