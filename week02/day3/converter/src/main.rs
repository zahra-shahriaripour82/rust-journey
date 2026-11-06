use std::io;

/// Reads one line from the keyboard, without spaces or the Enter key at the ends.
fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// Keeps asking until the user types a valid number.
fn read_number() -> f64 {
    loop {
        let input = read_line();
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("'{input}' is not a number, try again:"),
        }
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn km_to_miles(km: f64) -> f64 {
    km * 0.621371
}

fn miles_to_km(miles: f64) -> f64 {
    miles / 0.621371
}

fn kg_to_pounds(kg: f64) -> f64 {
    kg * 2.20462
}

fn pounds_to_kg(lb: f64) -> f64 {
    lb / 2.20462
}

fn main() {
    loop {
        println!();
        println!("=== Unit converter ===");
        println!("1) Celsius → Fahrenheit");
        println!("2) Fahrenheit → Celsius");
        println!("3) Kilometers → Miles");
        println!("4) Miles → Kilometers");
        println!("5) Kilograms → Pounds");
        println!("6) Pounds → Kilograms");
        println!("q) Quit");
        println!("Choose:");

        let choice = read_line();

        match choice.as_str() {
            "1" => {
                println!("Temperature in °C:");
                let c = read_number();
                println!("{c}°C = {:.1}°F", celsius_to_fahrenheit(c));
            }
            "2" => {
                println!("Temperature in °F:");
                let f = read_number();
                println!("{f}°F = {:.1}°C", fahrenheit_to_celsius(f));
            }
            "3" => {
                println!("Kilometers into Miles");
                let km: f64 = read_number();
                println!("{km} = {:.1} miles", km_to_miles(km))
            }
            "4" => {
                println!("  Miles into Kilometers ");
                let miles: f64 = read_number();
                println!("{miles} = {:.1} km", miles_to_km(miles))
            }
            "5" => {
                println!("Kilogram into pounds");
                let kg: f64 = read_number();
                println!("{kg} = {:.1} lb", kg_to_pounds(kg))
            }
            "6" => {
                println!("  pounds into Kilogram");
                let lb: f64 = read_number();
                println!("{lb} = {:.1} kg", pounds_to_kg(lb))
            }

            "q" => {
                println!("Bye!");
                break;
            }
            _ => println!("Unknown choice: '{choice}'"),
        }
    }
}
