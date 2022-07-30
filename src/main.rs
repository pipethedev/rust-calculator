use std::env;

fn main() {
    //let mut args: Args = args();
    let args: Vec<String> = env::args().collect();
    let first = args[1].clone();
    let operator: char = args[2].clone().chars().next().unwrap();
    let second = args[3].clone();

    // if first == "" || second == "" {
    //     println!("Please provide two numbers and an operator");
    //     return;
    // }

    // let first: String = args.nth(1).unwrap();
    // let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    // let second: String = args.nth(0).unwrap();
    println!("Values : {:?}", first); 

    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);

    println!("Result : {:?}", output(first_number, operator, second_number, result)); 
} 

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => return first_number + second_number,
        '-' => return first_number - second_number,
        '*' | 'x' | 'X' => return first_number * second_number,
        '/' => return first_number / second_number,
        _ => panic!("Invalid operator used"),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
}