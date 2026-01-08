fn main(){
    // Control Flow

    // If Statement
    // if condition {statemen1;statement2; ... }
    // no () parentheses
    let n = 10;
    if n==0 {println!("0");} // if 
    else { // else
        // nested if else
        if n>0 {println!("+ve");}
        else if n<0 {println!("-ve");} // else if
    }

    // in rust : condition value must be boolean, nothing else
    // if n{} // NO // error[E0308]: mismatched types, expected `bool`, found integer
    
    // using if-else statement inside let statement
    let over_rated = true;
    let momo = if over_rated {10} else {12} ; //10

    // Loops
    // Rust has Three Types of Loops
    // Loop
    // While
    // For

    // Loop
    // Basic Infinite Loop
    // loop {
    //     println!("Hi");
    // }
    // Hi
    // Hi
    // Hi
    // Hi
    // ... Infinite times

    // Adding Break Statement
    loop {
        println!("Hi");
        break ; // Break , Breaks the loop
    }
    // Hi

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter==5{
            break counter; // Break and return the value 
        }
    }; // result = 5
    println!("{}",result); 
    // 5

    // While
    // Conditional Loop
    let mut number = 3;
    
    while number != -3 { println!("{}",number);number-=1;}
    // number--; // NO, error: Rust has no postfix decrement operator
    // 3
    // 2
    // 1
    // 0
    // -1
    // -2

    // FOR LOOP - Iterator Loop
    let arr = [1,2,-3];
    for i in arr.iter(){println!("{}",i);}
    // 1
    // 2
    // -3
    for i in arr {println!("{}",i);}
    // 1
    // 2
    // -3

    // Looping Through Ranges
    // The last number is **exclusive** (not included)
    // `1..4` creates sequence: 1, 2, 3 (not 4)
    // Parentheses are optional around the range
    for i in {1..5} {println!("{}",i)}
    for i in 1..5 {println!("{}",i)}
    // 1
    // 2
    // 3
    // 4
    // 1
    // 2
    // 3
    // 4
}