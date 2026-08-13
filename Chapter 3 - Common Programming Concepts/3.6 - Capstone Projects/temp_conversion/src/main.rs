fn main() {
    println!("Enter a temperature to convert (e.g. 32F or 100C):");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input.len() < 2 {
        println!("Invalid input. Please enter a temperature followed by a unit (C or F).");
        return;
    }

    let (temp_str, unit) = input.split_at(input.len() - 1);

    let temp: f64 = match temp_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a valid number.");
            return;
        }
    };

    let converted_temp = match unit.to_uppercase().as_str() {
        "C" => (temp * 9.0 / 5.0) + 32.0, // Celsius to Fahrenheit
        "F" => (temp - 32.0) * 5.0 / 9.0, // Fahrenheit to Celsius
        _ => {
            println!("Invalid unit. Please use 'C' for Celsius or 'F' for Fahrenheit.");
            return;
        }
    };

    let converted_unit = if unit.to_uppercase() == "C" { "F" } else { "C" };
    println!("{}{} is equal to {:.2}{}", temp, unit.to_uppercase(), converted_temp, converted_unit);
}
