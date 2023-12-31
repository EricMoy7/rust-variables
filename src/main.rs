use dialoguer::{Input, Select};

fn main() {
    // Main menu selection screen
    let choices = vec!["Convert Temperature", "Nth term in Fibonacci Sequence", "12 Days of Christmas"];
    let user_input = Select::new()
        .items(&choices)
        .interact()
        .unwrap();
        
    match user_input {
        0 => temp_conv_seq(),
        1 => fibonacci_sequence(),
        2 => christmas_12_days(),
        _ => {
            println!("Input Error, please try again!");
            main()
        }
    }
}

// Sequences
fn temp_conv_seq() {
    let items = vec!["Fahrenheit", "Celsius"];
    let type_input = Select::new()
        .with_prompt("Select your input type")
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

fn fibonacci_sequence() {
    let input: u32 = Input::new()
    .with_prompt("Enter nth term")
    .interact_text()
    .unwrap();

    let fib_value: u32 = fibonacci(input);

    println!("{fib_value} is term number {input} in the Fibonacci sequence" )
}

fn christmas_12_days() {
    let day_numbers: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    
    let lyrics: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
        
        ];
        
    for i in 0..day_numbers.len()-1 {
        println!("On the {} day of Christmas", &day_numbers[i]);
        println!("My true love sent to me: ");
        let stuff = &lyrics[0..i+1];
        for (j, line) in stuff.iter().enumerate() {
            let slice_length = stuff.len()-1;

            // weird solution
            if i != 0 {
                if j == &slice_length-1 {
                    println!("{} and", &line);
                } else {
                    println!("{}", &line);
                }
            } else {
                println!("{}", &line);
            }
        }
        println!("");
    }

 
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

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}


