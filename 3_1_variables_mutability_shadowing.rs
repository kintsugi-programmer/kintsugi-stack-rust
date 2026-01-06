// use std::io;
fn main(){
    // Variables and Mutability

    // Vars in Rust are immutable by default 
    // also will throw warning as it's never used in code
    // warning: unused variable: `val1`
    let val1 = 10; 
    
    // you cannot re-assign immutable entity
    // error[E0384]: cannot assign twice to immutable variable `val1`
    // val1 = 20; // No
    
    // Mutable Variable, just by declare `mut` during initialization 
    // also will throw warning as it's(this assignment) never used in code( even thou we used it to print, but print uses the 2nd assigned value, not 1st )
    // warning: value assigned to `val2` is never read
    let mut val2 = 20; 

    // Re-assigned Successfully 
    // also not throw warning as we use this assignment (i.e. 2nd) to println
    val2 = 30; 
    println!("{}",val2);
    // 30

    // unused variable warning fix
    // if this is intentional, prefix it with an underscore: `_val1`
    let _val3 = 10;

    // Constants 
    // Contant values
    
    // - Constant naming convention: 
    //  - ALL CAPS
    //  - underscores(_) replacing spaces( ) 
    // eg: "Fixed value"(normal word intention) -> FIXED_VALUE" (Constant Name)
    const FIXED_VALUE: u32 = 1;// YES
    
    // can't use `mut` keyword in constants
    // error: const globals cannot be mutable
    // const mut FIXED_VALUE =1; // No

    // must be datatype annotated
    // error: missing type for `const` item, help: provide a type for the constant
    // const FIXED_VALUE =1; // No

    // numeric literals readability feature  
    // while storing numbers, we can use underscores as commas for readability
    // eg: 1000 = 1___000 = 10_00__  at rust 
    // eg: _1000 is wrong at rust 
    // purpoes example; eg: 1lakh, 100000, we use commas in english for readability 1,00,000.
    // at rust; to improve readability; 100000 = 1_00_000
    const FIXED_VALUE_1: u128 = 1_00_000 ;
    println!("{}",FIXED_VALUE_1); // 100000
    
    // constants may be set only to a constant expression
    // , not the result of a value that could only be computed at runtime. 
    const FIXED_VALUE_2: u128 = 1+ FIXED_VALUE_1; // allowed, computation done at compile time
    println!("{}", FIXED_VALUE_2);
    const FIXED_VALUE_3: u128 = 100*20000 + 10; // allowed, computation done at compile time
    println!("{}", FIXED_VALUE_3);

    // error[E0435]: attempt to use a non-constant value in a constant
    // computation done at runtime not allowed at const. assign

    // // example for computation at runtime
    // let mut input:u128 ;
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read input");
    // // const FIXED_VALUE_4: u128 = 1+ input; // NO, not allowed

    // Shadowing
    // Way to create a new variable using existing name
    // helps to preserve mutability and even update the value
    let x =1;// x is immutable,
    // x=2; // NO, error[E0384]: cannot assign twice to immutable variable `x`
    let x=2;// YES, Shadowing, x is still immutable, mutability preserved
    println!("{}",x);// 2
    let x="two";// YES, Shadowing, x is still immutable, mutability preserved, and typecasting
    println!("{}",x);// two
    let x=3;// YES, Shadowing, x is still immutable, mutability preserved
    println!("{}",x);// 3

    // purpoes example: to typecast data
    let x = x.to_string(); // Shadowing to Typecast int -> string
    println!("{}",x);// 3
    
    // clone()
    // we can't use x as x is chaged as string ops. 
    // so we use copy of x to play with it
    // println!("{}",x+"10");// 310 // NO, it will work at this line, after this line if used somewhere it will throw error, error[E0382]: borrow of moved value: `x`
    println!("{}",x.clone()+"10");// 310 // YES
    
}