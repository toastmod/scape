pub fn map(value: f32, rg1: (f32, f32), rg2: (f32, f32)) -> f32 {
    let rg1unit = rg1.1 - rg1.0; // eq 1 unit
    let rg2unit = rg2.1 - rg2.0; // eq 1 unit

    let scaledif = rg1unit / rg2unit;

    let value_inunits = value / rg1unit; //value in rg1 units

    value_inunits * scaledif
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    let mut tmp = 0f32;
    if input < min {
        return min
    }
    if input > max {
        return max
    }
    input
}