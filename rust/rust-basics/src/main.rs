fn main() {
    // declaring variable
    let x: i32 = 42; // immutable
    println!("this variable is immutable {}", x);

    // variables are constant by default
    // use the 'mut' keyword to define a variable that is mutable
    let mut mutable_var = 35;
    println!("this variable is mutable {}", mutable_var);
    mutable_var += mutable_var;
    println!("this variable has mutated {}", mutable_var);

    after_main();

    let pair = ('a', 17);
    println!("first element in tuple {}", pair.0);
    println!("second element in tuple {}", pair.1);

    let array = vec![1, 2, 3, 4];
    println!("this is a vector {:?}", array);
}

fn after_main() {
    println!("this function is defined after the main function");
}
