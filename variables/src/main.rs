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


    // Scalar Types
    let value1: i8 = 1; // u8 for unsigned
    let value2: i16 = 1; // u16 for unsigned
    let value3: i32 = 1; // u32 for unsigned
    let value4: i64 = 1; // u64 for unsigned
    let value5: i128 = 1; // u128 for unsigned
    let value6: isize = 1; // based on os architecture 32 or 64, usize for unsigned

    let float1: f32 = 2.0;
    let float2: f64 = 2.0;

    let c1 = 'z';
    let c2: char = 'A';
    let heart_eyed_cat = '😻';
    // let c2 = "z"; it is string not character
    // let c: char = "z"; ERROR!

    // number literal 
    let decimal_literal = 10_000;
    let hex_literal = 0xff;
    let octal_literal = 0o77;
    let binary_literal = 0b10;
    let byte_literal = b'A';


    // => Compound Types (Tuple, Array)
    
    // Tuple
    // The tuple without any values has a special name, unit. 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (n, o, p) = tup; // destructuring
    println!("value of o is {o}");
    let third_element = tup.2;
    println!("third element of tup is {third_element}");

    
    // Array
    let array = [1, 2, 3, 4, 5];
    let arr = [5; 10];
    let first = arr[0];
    println!("first is {first}");
}
