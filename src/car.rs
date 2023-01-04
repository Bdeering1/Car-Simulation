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


struct Engine {

}

impl Engine {
    // t = (x-1000) / 7000
    // y = (v1)(1-t)^3 + (v2)(3t)(1-t)^2 + (v3)(3t^2)(1-t) + (v4)(t^3)
    // v1 = 200, v2 = 274, v3 = 380, v4 = 225
    
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