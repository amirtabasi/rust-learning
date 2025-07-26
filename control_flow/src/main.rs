fn main() {
    let number = 10;

    if number != 0 {
        println!("number is not zero");
    } else {
        println!("number is zero");
    }

    let value = if number == 10 { 50 } else { 0 };
    println!("value is {value}");

    let mut counter = 0;

    // assign value using loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");
}
