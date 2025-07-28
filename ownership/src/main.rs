/*
The Stack and the  Heap
both are parts of memory available to your code to use at runtime.
the stack stores values in the order it gets them and removes the values in the opposite order.(last in, first out)
adding data to the stack is called pushing onto the stack, and removing data is called popping off the stack.
all data stored on the stack must have a known, fixed size.
data with an unknown size at compile time or a size that might change must be stored on the heap instead.
when you put data on the heap, you request a certain amount of space.
the memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
this process is called allocating on the heap or just allocating.
because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
Accessing data in the heap is slower than accessing data on the stack becauses you have to follow a pointer to get there.

When your code calls a function, the values passed into the function (including potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. when the function is over, those values get popped off the stack.

1- Keeping track of what parts of code are using what data on the heap
2- minimizing the amount of duplicate data on the heap
3- cleaning up unused data on the heap so you don't run out of space
These are all problems taht ownership addresses.
*/

fn main() {
    // let s  = "hello world" // this will save on the stack
    let mut st = String::from("Hello world"); // this will save on the heap
    st.push_str(", what a good day!");
    println!("{st}");


    // because these are known and fixed size, they will save on the stack
    let x = 5;
    let y = x;

    // A String is made up of three parts which stored on the stack
    // a pointer to the memory that holds the contents of the string
    // a length of string
    // a capacity
    let s1 = String::from("hello");
    // this copy values from the stack, so both s1 and s2 point to the same location on the heap
    let s2 = s1;

    // because both s1 and s2 point to the same location on memory,
    // when both of them go out of scope, they try to free the same memory.
    // this is known as a double free error

    // println!("{s1}"); // it paincs because after the line let s2 = s1; Rust considers s1 as no longer valid.

    // shallow copy = copying the pointer, length and capacity without copying the data is shallow copy
    // but because Rust also INVALIDATE the first variable, instead of being callled a shallow copy, it's known as a MOVE.
    // s1 was moved into s2;


    // when you assign a completely new value to an existing variable, Rust will call drop and free the original value's memory immediately.
    let mut s = String::from("Hello");
    s = String::from("ahoy");


    // Deep copy

    let t1 = String::from("hello");
    let t2 = t1.clone();
    println!("t1 = {t1}, t2 = {t2}");

    let v1 = String::from("this is a text");
    takes_ownership(v1); // v1 moves into the function
    // println!("v1 is {v1}"); v1 is no longer valid here!

    let v2 = 5;
    makes_copy(v2); // because i32 implements the Copy trait, v2 does not move into the function
    println!("v2 is {v2}"); // it's okay to use v2 afterward


    let my_string = gives_ownership();

    let another_string = String::from("hello");

    let last_string = takes_and_gives_back(another_string);

    // at the end of the main scope, last_string goes out of the scope 
    // and is dropped. another_string was moved, so nothing happens
    // my_string goes out of scope and is dropped.


}


fn takes_ownership(value: String) {
    println!("Value is {value}");
}

fn makes_copy(value: i32) {
    println!("value is {value}");
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}