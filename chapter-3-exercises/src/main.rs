fn main() {
    let value = std::env::args().nth(1).expect("No degree value given!");
    let current_unit = std::env::args().nth(2).expect("No unit given!");

    if current_unit == "C" {
        println!("{} degrees Celsius is:", value);
        println!(
            "{} degrees Fahrenheit",
            celsius_to_fahrenheit(value.parse::<i32>().unwrap())
        );
    } else {
        println!("{} degrees Fahrenheit is:", value);
        println!(
            "{} degrees Celsuis",
            fahrenheit_to_celsius(value.parse::<i32>().unwrap())
        );
    }
}

fn celsius_to_fahrenheit(cel_temp: i32) -> i32 {
    (cel_temp * 9) / 5 + 32
}

fn fahrenheit_to_celsius(fahren_temp: i32) -> i32 {
    ((fahren_temp - 32) * 5) / 9
}
