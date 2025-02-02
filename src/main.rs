use std::io;

fn main() {

    loop {
        println!("Please input");
        println!("0 for converting temperatures from Fahrenheit to Celsius");
        println!("Other number for converting temperatures from Celsius to Fahrenheit");
        println!("Non numberic to quit");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("Please input your temperature.");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");

        let mut from_unit = "Celsius"
        let mut to_unit = "Fahrenheit"

        let conversion: u32 = match temperature.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => break,
        };

        match guess {
            0 => println!("Your conversion value is {}", (conversion-32)*5/9),
            _ => println!("Your conversion value is {}", conversion*9/5+32),
        }
    }
}
