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
    'counting_up: loop { // Loop label
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

    let mut number = 10;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("end while!");



    let arr = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("the value is: {}", arr[i]);
        i += 1;
    }

    for element in arr {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }


}
