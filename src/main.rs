mod car;

fn main() {
    let mut e: car::Engine = car::Engine::new(1000, 8000, (200.0, 274.0, 380.0, 225.0));

    e.set_rpm(5500);

    println!("Engine: {}", e.get_torque());
}
