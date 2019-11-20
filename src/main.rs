fn main() {
    println!("Fibonacci Number Calculator by tekwizz123");
    let mut input_string = String::new();
    let mut input_number: i32 = -22;

    loop {
        input_string.clear();
        println!("Enter the position in the Fibonacci sequence that you want to calcuate the number of: ");
        let length_counter = match std::io::stdin().read_line(&mut input_string) {
            Result::Ok(a) => a,
            Result::Err(_) => 0,
        };
        if length_counter >= 1 {
            println!("Converting input...");
            input_number = match input_string.trim().parse() {
                Result::Ok(a) => a,
                Result::Err(_) => -1,
            };
        }
        if input_number >= 0 {
            break;
        } else {
            println!("ERROR: Please enter a positive number!");
            println!();
        }
    }
    println!(
        "Input number: {}",
        calcuate_fibonacci_given_number(input_number)
    );
}

fn calcuate_fibonacci_given_number(number: i32) -> i32 {
    let mut current_number = 1;
    let mut previous_number = 0;
    let mut temp_current_number_holder: i32;
    let mut loop_counter: i32 = number - 2;
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }
    while loop_counter >= 0 {
        temp_current_number_holder = current_number;
        current_number += previous_number;
        previous_number = temp_current_number_holder;
        loop_counter -= 1;
    }
    current_number
}
