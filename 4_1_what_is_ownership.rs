fn main(){

// Ownership 
// Replaces the garbage collector
// New way to manage memory in rust

// Garbage Collector
// java & c#
// error free, faster write time
// no control, slow runtime, unpredictable performance, larger program size

// Manual Memory Management
// C & C++
// Full Control, Faster Runtime, Small Program Size
// Error Prone Extreme, Slow Write time

// Ownership Model
// Rust
// Control over memory, Faster Runtime, Smaller Program size, Error Free
// Slow Write time, Learning Curve

// Memory

// Stack Memory
// Fixed Size
// each func -> stack frame gen., size cal. at compile time, vars inside stack => same size, variable live as long as stack lives

// Heap Memory
// Dynamic Size, More Flexible, Less Organized
// Lifetime of data can be controlled

// Stack Vs Heap Example
   fn a(){ // a executed first
        let x: &str = "Hello World";// Static // a's Stack frame add var
        let y: u128 = 255;// Static // a's Stack frame add var
        b(); // b executed
    }//a's Stack Flushed with all it's variables

    fn b(){
        // Push stack frame for 'b' onto stack
        let x: String = String::from("Momo"); // Dynamic
        // Can't store directly on stack
        // Heap allocates memory for string
        // Heap returns a pointer
        // Pointer is what's stored on stack

        // When 'b' finishes, it gets popped off stack
        // All variables of 'b' are dropped
    }
// Push to Stack is faster than Heap Allocation
// Accessing Stack is Faster than Heap allocation
// Heap is Slow than Stack in everything, but It has Control of Lifetime of Variable & Stack Variables are Fixed; Heap Variables are Dynamic

// THE 3 OWNERSHIP RULES
// 1 Each Var has One Owner
// 2 One Var = Only One Owner
// 3 When the Owner goes out of scope, the value is dropped

// Ownership Example - Scope
{   // New Scope Created

    let s: &str = "Hello World!!!"; // String Literal
    // s is Created
    // s is valid from this point
    // we can play with s

}   // scope Ended, s Invalidated, Rust Drop it 

// String Literal vs String Type
// String Literal: Stored directly in binary, fixed size
// String Type: Dynamic Size, Stored in Heap
{   // New Scope Created

    let s: String = String::from("Hello World!!!"); // String Type
    // s is Created
    // s is valid from this point
    // we can play with s

}   // scope Ended, s Invalidated, Rust Drop it 

// In Rust, Allocation Happens Automatically During Declaration, DeAllocation Happens Automatically when Scope Ends ,Unlike C++(where `new`&`delete` keywords are required)

// Interaction Between Varibles and Data 

// Simple Copy
// Copy Trait
let x = 5;
let y = x; // Yes, Value of x is copied to y // This does a COPY, not a move
println!("{}", x);  // x is still valid 
// 5

// String Move
let x = String::from("hi");
// x under the hood:
// - pointer: points to memory location on heap
// - length: length of the string
// - capacity: actual amount of memory allocated on heap
let y = x; // Yes, But it Doesn't Clone the Value, it doesn't Shallow Copy(Both Vars point to same Val), Actually y takes ownership from x, making x invalidated and y is new owner of data
println!("{}",y);
// hi
// println!("{}",x); // No
// error[E0382]: borrow of moved value: `x`

// Memory Safety Feature by rust
// Rust defaults to Move Values

// If Actually wanna clone the String Type
let x = y.clone(); // Explicitly clone
println!("{}",x);// hi
println!("{}",y);// hi
// Now Both are working

// Ownership and Functions

// Example: Moving into Functions
let s = String::from("Hi");
takes_ownership(s);// when s moved in function, function takes its ownership
// Hi
// Passing parameters into a function is the same as assigning to another variable

// println!("{}",s); // NO, error[E0382]: borrow of moved value: `s`

// Example: Copying into Functions
let s: u128 = 10;
makes_copy(s);// Ownership not given to function, Value Copied into it
// Copy trait is implemented
// 10
println!("{}",s);// Yes Works fine! x is still valid

// Returning Values and ownership

// Example: Giving Ownership
let s = gives_ownership(); // Function Gives Ownership to Another Var
println!("{}",s);// Yes 
// Hello

// Example: Taking and Giving Back
let s = String::from("Hayabusamaru");
let s1 = takes_and_gives_back_ownership(s);
// Function takes Ownership from Arguement, and then gives it as Return

// println!("{}",s);// error[E0382]: borrow of moved value: `s`
println!("{}",s1);
// Hayabusamaru

}

fn takes_ownership(some_string: String){println!("{}",some_string);}

fn makes_copy(some_u128: u128){println!("{}",some_u128);}

fn gives_ownership() -> String{
    let some_string = String::from("Hello");
    some_string // Return moves ownership out of function
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
    // Function takes Ownership from Arguement, and then gives it as Return
    some_string // Return moves value back out
}