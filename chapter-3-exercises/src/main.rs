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
        "1" => temperature_converter(),
        "2" => fibonacci_generator(),
        "3" => {
            println!("Twelve Days of Christmas");
        }
        _ => {
            println!("Operation not found: {}", selection.as_str());
        }
    }
}

fn temperature_converter() {
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

fn celsius_to_fahrenheit(cel_temp: i32) -> i32 {
    (cel_temp * 9) / 5 + 32
}

fn fahrenheit_to_celsius(fahren_temp: i32) -> i32 {
    ((fahren_temp - 32) * 5) / 9
}

fn fibonacci_generator() {
    println!("Which Fibonacci number do you want?");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = num.trim().parse::<i32>().unwrap();
    let mut generated_nums: Vec<i32> = Vec::with_capacity(num as usize);
    generated_nums.push(0);
    generated_nums.push(1);

    for i in 2..num {
        let first_index = i as usize - 1;
        let second_index: usize = i as usize - 2;
        let new_num = generated_nums[first_index] + generated_nums[second_index];
        generated_nums.push(new_num);
    }
    println!(
        "#{} in the Fibonacci sequence is: {}",
        num,
        generated_nums.last().unwrap()
    )
}
