fn main(){
    // Data Types
    // Scaler datatypes represent single value
    // Compound datatypes represent group of values

    // Scaler datatypes
    // Integers
    // Floating-point numbers
    // Booleans
    // Character

    // Integers
    // numbers without fractional component
    // Signed:      either +ve/-ve integers
    // unsigned:    only +ve integers
    
    // Integer Types in Rust
    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // Architecture-dependent	isize	usize

    // Architecture-dependent: the isize and usize types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // i32 is default in rust integer datatype

    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte ( u8 only )

    // Integer Overflow 
    let f: u8 = 255; // max value of u8
    // let g: u8 = 256; // Integer Overflow
    // at debug build -> Rust Panics (error: literal out of range for `u8`)
    // at release build(compiling in release mode with the --release flag) -> Rust performs 2's Complement Wrapping
    // Rust performs 2's Complement Wrapping => number exceeding max value will wrap around minimum value
    // 256 -> 0+1 -> 1
    // let h: u8 = 260; // 260 -> 0+5 -> 5

    // btw
    // cargo build --release # release flag
    // rustc -O main.rs # optimization flag 

    // Floating-point numbers
    // Decimal values
    let i = 6.8; // default is f64, 64 bit ; double precision floating point number
    let j:f32 = 6.9; // f32
    // The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. 
    // All floating-point types are signed.
    // Floating-point numbers are represented according to the IEEE-754 standard.
    
    // Numeric Operations
    let sum = 5+5333; // Addition
    let div_quotient = 55/2; // Division Quotient
    let div_truncated = -5/3; // Division Truncated
    // Results in -1 
    let remainder = 10%2; // Remainder
    let mul = 55.5 * 100.222; // Multiplication
    let sub = 0.0 - 10.99; // Subtraction

    // Boolean
    // two possible values: true and false.
    // Booleans are one byte in size.
    let k = true;
    let l:bool = false;

    // Character 
    // Rust’s char type is 4 bytes in size
    // Unicode Values
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    // Compund Types
    // Represent a group of Values
    // The Tuple Type
    // The Array Type

    // The Tuple Type ()
    // Fixed Size Array
    // Different Datatype Elements
    let tup = ("Hello World",102_00, 1001.11,'😻', true);
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    // Tuple Access Ways
    // Tuple Access Ways: destructuring & dot notation
    // destructuring way
    let (val1,val2,val3) = tup1;
    // dot notation way
    let val1_1 = tup1.0; 
    // tuple index start from 0
    let val2_1 = tup1.1;
    let val3_1 = tup1.2;

    // The Array Type []
    // Fixed Size Array
    // Same Datatype Elements
    let a = [1,2,3];
    let b = [1.01,2.12212,3.0];
    // let array_out_of_bound_err_exception_eg = b[3]; // NO, index out of bounds
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let curr_month = months[0]; // Array Access
    let a = [-20; 3]; // Array Initialize Multiple at once [Value;Occurences]
    // [-20, -20, -20]

    // Rust Prevents us to do anything Memory Unsafe !!!


}