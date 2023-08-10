use std::io;

fn main() {
    temp_conv_seq()
}

// Sequences
fn temp_conv_seq() {
    let f_temp: f32;
    let c_temp: f32;
    let mut input = String::new();

    println!("Please enter a temperature in Farenheit you want to convert: ");
    io::stdin().read_line(&mut input).expect("Not a valid input");

    f_temp = input.trim().parse().expect("Not a valid number was inputted");

    c_temp = convert_farenheit_celcius(f_temp);

    println!("{}",c_temp)
}

// Helper functions
fn convert_farenheit_celcius(temperature_f: f32) -> f32 {
     let temp_c: f32 = (temperature_f - 32.0) * (5.0/9.0);
     temp_c
}