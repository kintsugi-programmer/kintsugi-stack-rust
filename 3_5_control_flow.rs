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

}