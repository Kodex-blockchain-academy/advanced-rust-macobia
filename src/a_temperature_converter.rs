//Temperature Converter
// Build a CLI program that:
// 1. Converts temperatures between Celsius, Fahrenheit, and Kelvin.
// 2. Validates user input and displays an error message for invalid data.


use std::io::{self, Write};

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Function to convert Celsius to Kelvin
fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Function to convert Fahrenheit to Kelvin
fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    fahrenheit_to_celsius(fahrenheit) + 273.15
}

// Function to convert Kelvin to Celsius
fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

// Function to convert Kelvin to Fahrenheit
fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}

// Function to handle user input and validate it
fn get_temperature_input() -> Result<f64, &'static str> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input. Please enter a valid number."),
    }
}

// Main function to handle CLI interaction
fn main() {
    loop {
        println!("Temperature Converter:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Celsius to Kelvin");
        println!("3. Fahrenheit to Celsius");
        println!("4. Fahrenheit to Kelvin");
        println!("5. Kelvin to Celsius");
        println!("6. Kelvin to Fahrenheit");
        println!("7. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");

        match option.trim() {
            "1" => {
                println!("Enter temperature in Celsius:");
                match get_temperature_input() {
                    Ok(celsius) => {
                        let fahrenheit = celsius_to_fahrenheit(celsius);
                        println!("{:.2} Celsius = {:.2} Fahrenheit", celsius, fahrenheit);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "2" => {
                println!("Enter temperature in Celsius:");
                match get_temperature_input() {
                    Ok(celsius) => {
                        let kelvin = celsius_to_kelvin(celsius);
                        println!("{:.2} Celsius = {:.2} Kelvin", celsius, kelvin);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "3" => {
                println!("Enter temperature in Fahrenheit:");
                match get_temperature_input() {
                    Ok(fahrenheit) => {
                        let celsius = fahrenheit_to_celsius(fahrenheit);
                        println!("{:.2} Fahrenheit = {:.2} Celsius", fahrenheit, celsius);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "4" => {
                println!("Enter temperature in Fahrenheit:");
                match get_temperature_input() {
                    Ok(fahrenheit) => {
                        let kelvin = fahrenheit_to_kelvin(fahrenheit);
                        println!("{:.2} Fahrenheit = {:.2} Kelvin", fahrenheit, kelvin);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "5" => {
                println!("Enter temperature in Kelvin:");
                match get_temperature_input() {
                    Ok(kelvin) => {
                        let celsius = kelvin_to_celsius(kelvin);
                        println!("{:.2} Kelvin = {:.2} Celsius", kelvin, celsius);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "6" => {
                println!("Enter temperature in Kelvin:");
                match get_temperature_input() {
                    Ok(kelvin) => {
                        let fahrenheit = kelvin_to_fahrenheit(kelvin);
                        println!("{:.2} Kelvin = {:.2} Fahrenheit", kelvin, fahrenheit);
                    }
                    Err(e) => println!("{}", e),
                }
            }
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
