use std::io;

fn main() {
    println!(
        "Select an operation:
         1| Temperature Converter
         2| Fibonacci Generator
         3| Twelve Days of Christmas
        "
    );

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    match selection.as_str().trim() {
        "1" => {
            println!("Input temperature in degrees:");
            let mut degrees = String::new();

            io::stdin()
                .read_line(&mut degrees)
                .expect("Failed to read line");
            let degrees: i32 = degrees.trim().parse::<i32>().unwrap();

            println!("Input unit (F, C):");
            let mut unit = String::new();

            io::stdin()
                .read_line(&mut unit)
                .expect("Failed to read line");

            match unit.as_str().trim() {
                "C" => {
                    println!(
                        "{} degrees Celsius is: {} F",
                        degrees.to_string(),
                        celsius_to_fahrenheit(degrees)
                    )
                }
                "F" => {
                    println!(
                        "{} degrees Fahrenheit is: {} C",
                        degrees.to_string(),
                        fahrenheit_to_celsius(degrees)
                    )
                }
                _ => {
                    println!("Unit not found!")
                }
            }
        }
        "2" => {
            println!("Fibonacci Generator");
        }
        "3" => {
            println!("Twelve Days of Christmas");
        }
        _ => {
            println!("Operation not found: {}", selection.as_str());
        }
    }
}

fn celsius_to_fahrenheit(cel_temp: i32) -> i32 {
    (cel_temp * 9) / 5 + 32
}

fn fahrenheit_to_celsius(fahren_temp: i32) -> i32 {
    ((fahren_temp - 32) * 5) / 9
}
