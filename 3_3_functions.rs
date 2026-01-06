fn main(){
    func_1(); // simple function call
    // Hello World !!!
    func_2(22,"Kintsugi-programmer"); // function call with arguements
    // Hi Kintsugi-programmer, you type 22
    println!("{}",func_3(1,10));
    println!("{}",func_3_1(1,10));
    let sum = func_3_2(1,10);
    println!("{}",sum);
    let val = five();
    println!("{}",val);
}

// Functions
// Functions Convention in Rust : snake_case 
// in rust, Function code is either 
// statement(do action and no return) 
// or expression(do action and return some value)
fn func_1(){println!("Hello World !!!");} // simple function // Statement
fn func_2(x:u128, y:&str){println!("Hi {}, you type {}",y,x);} // Statement

// return, in Expression Functions
fn five() -> i32 {
    5
}

// return ways

// return way 1
fn func_3(x:u128, y:u128) -> u128{ // specify return type
    println!("Hi"); // statement
    let sum = x+y;
    return sum; // return keyword
}

// return way 2
fn func_3_1(x:u128, y:u128) -> u128{ // specify return type
    println!("Hi");
    let sum = x+y;
    sum // remove return keyword AND remove semicolon to return this value
    // rust func will return this last expression
    // this not statement, as we remove ;, and make it expression
}

fn func_3_2(x:u128, y:u128) -> u128{ // specify return type
    println!("Hi");
    x+y // more simplified version
}

// Hello World !!!
// Hi Kintsugi-programmer, you type 22
// Hi
// 11
// Hi
// 11
// Hi
// 11
// 5