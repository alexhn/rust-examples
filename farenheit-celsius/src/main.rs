use std::io;

fn main() {

    loop {

        println!("Input Farenheits:");

        let mut farenheit = String::new();

        io::stdin().read_line(&mut farenheit)
            .expect("Error");

        let farenheit: u32 = match farenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = farenheit_to_celsius(farenheit);

        println!("Celsius: {} ", celsius);
    }

}

fn farenheit_to_celsius(farenheit: u32) -> u32 {
    (farenheit - 32) * 5 / 9
}
