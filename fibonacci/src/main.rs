use std::io;

fn main() {
    let mut input = String::new();

    println!("Caclulating Fibonacci");
    println!("Please enter your number");

    io::stdin()
        .read_line(&mut input)
        .expect("please enter a number");

    let input: i32 = input.trim().parse().expect("please enter a valid number");

    let fib = fibonacci(input);
    println!("fib is {fib}");
}

/// #### Formula:
/// Fn = Fn-1 + Fn-2
///
/// f0 = 0
///
/// f1 = 1
///
/// f2 = f1 + f0 --- 1 + 0 = 1
///
/// f3 = f2 + f1 --- 1 + 1 = 2
///
/// f4 = f3 + f2 --- 2 + 1 = 3
///
/// f5 = f4 + f3 --- 3 + 2 = 5
///
/// f6 = f5 + f4 --- 5 + 3 = 8
///
/// f7 = f6 + f5 --- 8 + 5 = 13
///
/// f8 = f7 + f6 --- 13 + 8 = 21
///
/// and so on ...
fn fibonacci(number: i32) -> i32 {
    if number <= 0 {
        0
    } else if number == 1 {
        1
    } else {
        println!("calculating fib {number}");
        let x = fibonacci(number - 1) + fibonacci(number - 2);
        println!("fib {number} = {x}");
        x
    }
}
