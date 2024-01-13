/* Algorithm for conversion as shared by Google Bard

Conversion from Fahrenheit to Celsius:

    Start
    Input the temperature in Fahrenheit (F)
    Apply the formula: C = (5/9) * (F - 32)
    Output the temperature in Celsius (C)
    End

Conversion from Celsius to Fahrenheit:

    Start
    Input the temperature in Celsius (C)
    Apply the formula: F = (9/5) * C + 32
    Output the temperature in Fahrenheit (F)
    End
*/

use std::io;

fn read_user_choice() -> u8 {
    println!("What do you want to do ?");
    println!("1. Convert from Fahreinheit to Celsius");
    println!("2. Convert from Celsius to Fahreinheit");
    println!("3. Quit");
    let mut choice = String::new();
    println!("Please input your choice.");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Please enter numeric value");
    choice
}

fn get_input_temp() -> u8 {
    let mut temp = String::new();
    println!("Please input temparature :");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: u8 = temp.trim().parse().expect("Please enter numeric value");

    temp
}

fn c2f(input: u8) -> f32 {
    (9.0 / 5.0) * input as f32 + 32.0
}

fn f2c(input: u8) -> f32 {
    (5.0 / 9.0) * (input as f32 - 32.0)
}

fn main() {
    let choice: u8 = read_user_choice();
    println!("You chose: {choice}");
    let result: f32;
    let input;
    match choice {
        1 => {
            input = get_input_temp();
            result = f2c(input);
        }
        2 => {
            input = get_input_temp();
            result = c2f(input);
        }
        _ => return,
    }
    println!("Result: {}", result as i32);
}
