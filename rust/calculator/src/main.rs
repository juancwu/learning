use std::io::{self, BufRead};

fn eval_prefix(prefix: &str) {
    let mut number_arr: Vec<i32> = Vec::new();

    for c in prefix.chars().rev() {
        if let Some(num) = c.to_digit(10) {
            number_arr.push(num as i32);
        } else if "+-*/".contains(c) {
            let b = number_arr.pop().expect("Number Array Overflow.");
            let a = number_arr.pop().expect("Number Array Overflow.");

            let result = match c {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => unreachable!(),
            };

            number_arr.push(result);
        }
    }

    number_arr.pop().expect("Number Array Overflow");
}

fn main() {
    let mut input = String::new(); 
    let mut result;

    loop {
        input.clear();

        io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("Failed to read from console.");

        let trimmed_input = input.trim();
        if trimmed_input == "q" {
            println!("Exiting...");
            break;
        } else {
            println!("Your input is {input}");
            result = eval_prefix(&input);
            println!("Result: {:?}", result);
        }
    }

}
