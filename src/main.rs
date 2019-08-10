/*
    basic calculator for converting fahrenheit to celsius and celsius to fahrenheit.
*/
use std::io;
fn main() {
    println!("Welcome to the temperature converter!");
    loop {
        println!(
            "Please press '1' and 'enter' to convert to celcius and '2' and 'enter' for Fahrenheit"
        );
        let mut val = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read line");

        let val: u32 = match val.trim().parse() {
            Ok(1) => 1,
            Ok(2) => 2,
            _ => continue,
        };

        println!("Please enter the temperature value in numbers e.g: 32");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if val == 1 {
            let celsius: f64 = (temperature - 32.0) * (5.0 / 9.0);
            println!(
                "{} Fahrenheit converted to Celsius is {} Celsius.",
                temperature.round(),
                celsius.round()
            );
            break;
        }
        if val == 2 {
            let fahrenheit: f64 = (temperature * (9.0 / 5.0)) + 32.0;
            println!(
                "{} Celsius converted to Fahrenheit is {} Fahrenheit.",
                temperature.round(),
                fahrenheit.round()
            );
            break;
        }
    }
}
