use std::io;

fn main() {
    println!("Welcome to temperature conversion application");

    loop {
        println!("Temperature to convert: ");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read temperature input");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} is not a valid temperature", temperature.trim());
                continue;
            }
        };

        println!("[1] CELSIUS => FAHRENHEIT\n[2] FAHRENHEIT => CELSIUS\nSelect 1 or 2: ");
        let mut measurement = String::new();
        io::stdin()
            .read_line(&mut measurement)
            .expect("Failed to read measurement input");

        let measurement: i32 = match measurement.trim().parse() {
            Ok(num) => num,
            Err(_) => 1,
        };

        let celsius = fahrenheit_to_celsius(temperature);
        let fahrenheit = celsius_to_fahrenheit(temperature);

        if measurement == 1 {
            println!("{}°C = {1:.1}°F", temperature, fahrenheit);
        } else if measurement == 2 {
            println!("{}°F = {1:.1}°C", temperature, celsius);
        }

        break;
    }
}

// °C = (°F − 32) x 5/9
fn fahrenheit_to_celsius(temperature: i32) -> f64 {
    (f64::from(temperature) - 32.0) * 5.0 / 9.0
}

// °F = (°C × 9/5) + 32
fn celsius_to_fahrenheit(temperature: i32) -> f64 {
    (f64::from(temperature) * 9.0 / 5.0) + 32.0
}
