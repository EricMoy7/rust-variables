use dialoguer::{Input, Select};

fn main() {
    temp_conv_seq();
}

// Sequences
fn temp_conv_seq() {
    let items = vec!["Fahrenheit", "Celsius"];
    let type_input = Select::new()
        .items(&items)
        .interact()
        .unwrap();

    let type_input = items[type_input];


    let input: f32 = Input::new()
        .with_prompt(format!("Enter temperature in {}", type_input))
        .interact_text()
        .unwrap();


    let final_temp: (f32, &str) = fahrenheit_celcius(input, &type_input);

    println!("{} \u{00B0} {} is {} \u{00B0} {}", input, type_input, final_temp.0, final_temp.1)

    // final_temp
}

// Helper functions
fn fahrenheit_celcius(temp: f32, input_type: &str) -> (f32, &str) {
    if input_type == "Celsius" {
        let temp_f: f32 = (temp * 9.0/5.0) + 32.0;
        (temp_f, "Fahrenheit")
    } else if input_type == "Fahrenheit" {
        let temp_c: f32 = (temp - 32.0) * (5.0/9.0);
        (temp_c, "Celsuis")
    } else {
        println!("Invalid input type...");
        (0.0, "Error")
    }
}
