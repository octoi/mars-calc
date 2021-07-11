use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your weight (Kg): ");
    io::stdin().read_line(&mut input).unwrap();

    let weight_on_earth: f32 = input.trim().parse().unwrap();
    let weight_on_mars = calculate_weight_on_mars(weight_on_earth);

    println!("Weight on Mars: {}Kg", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
