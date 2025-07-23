const DELAY_IN_SECOND: u32 = 30;
// let delay = 30;
// let cannot be used for global variables

fn main() {
    println!("Delay in second is {DELAY_IN_SECOND}");

    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    // try remove mut from definition of x and see the output of cargo run
    println!("using mut keyword let me change value of x, x is {x}");

    const PI: f32 = 3.14;
    // try define a constant with lower case name and see the output of cargo run
    println!("PI is alwasy {PI}");

    const TWO: u32 = 1 + 1;
    println!("two is {TWO}");


    let x = 5;

    let x = x + 1; // Shadowing x
    
    {
        let x = x * 2;
        println!("the value of x in inner scope is: {x}");
    }

    println!("the value of x is: {x}");

    let spaces = "    "; // imagine user input this via commandline
    let spaces = spaces.len(); // now we have what we realy wanted, number of space character
    println!("number of spaces is {spaces}");
    // this will throw error because the type of spaces is string so we cannot put integer inside it.
    // let mut spaces = "    ";
    // spaces = spaces.len();


    let mut y = 1;
    {
        let mut y = y + 1;
        y += 2;
        println!("inner scope {y}");
    }
    println!("{y}");
}
