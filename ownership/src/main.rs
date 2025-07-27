
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
    println!("Hello, world!");
}
