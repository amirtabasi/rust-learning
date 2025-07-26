fn main() {
    // this is a statement
    let x = 1;

    // this is an expression
    let y = {
        let a = 3;
        a + 1 // here we dont use ; 
    };

    // this is an expresion
    let x = five();

    let sum = sum(10, 20);
    println!("10 + 20 = {sum}");

    println!("x is now {x}");

    println!("y is {y}");

    println!("Hello, world!");

    another_function(10, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("value is {value}{unit_label}")
}

fn five() -> i32 {
    println!("this function returns 5");
    5
}


fn sum(a: i32, b: i32) -> i32 {
    a + b // do not use ; for return value!
}
