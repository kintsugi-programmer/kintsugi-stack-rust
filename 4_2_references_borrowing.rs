// Reference and Borrowing

fn main(){

// References

    // The Problem with Ownership Transfer

    // Tedious Example
    let s1 = String::from("Buri Buri Zaemon");
    let len = cal_len(s1);
    // println!("The length of '{}' is {}.",s1,len); // NO // error[E0382]: borrow of moved value: `s1`
    // This code will not compile as-is because s1 is moved into calculate_length(s1), so it can’t be used again in println!.

    // Tedious Solution
    let s1 = String::from("Buri Buri Zaemon");
    let (s1, len) = cal_len_1(s1);// using s2 or redeclaring s1 here
    println!("The length of '{}' is {}.",s1,len); // The length of 'Buri Buri Zaemon' is 16.
    // Have to return tuple to give ownership back
    // Very strange looking

    // Better Solution
    // References
    // References allow you to use a value without taking ownership.
    // - Use `&` to create a reference
    // - References don't take ownership of the underlying value
    // - When reference goes out of scope, the underlying value is NOT dropped
    let s1 = String::from("Buri Buri Zaemon");
    let len = cal_len_2(&s1); // Pass reference to s1
    println!("The length of '{}' is {}.",s1,len);  // s1 still valid
    // The length of 'Buri Buri Zaemon' is 16.

// Borrowing 
    // - Passing references as function parameters
    // - You're borrowing the value
    // - Not taking ownership of it


// References are immutable by default
    let s = String::from("Hello");
    // change(&s); // No // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference

// Mutable References
    // Steps to use mutable references:
    // 1. Make the variable mutable with `mut`
    // 2. Pass mutable reference with `&mut`
    // 3. Function takes mutable reference with `&mut`
    let mut s = String::from("Hello"); // 1. Make the variable mutable with `mut`
    change(&mut s); // Yes // 2. Pass mutable reference with `&mut`
    println!("{}", s); // Hello, world !!!

// Mutable Reference Restriction
    // Only ONE mutable reference to a piece of data in a scope is allowed
    // - Prevents **data races** at compile time
    
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // No // ERROR
    // println!("{}, {}", r1, r2); // No // error[E0499]: cannot borrow `s` as mutable more than once at a time
    
    println!("{}", r1); //hello
    
    // A data race occurs when:
    // 1. Two pointers point to the same piece of data
    // 2. One pointer is used to write to the data
    // 3. There's no mechanism to synchronize data access between pointers
    // 4. Result: One pointer tries to read data while the other is modifying it
    // 5. Outcome: Corrupt data

// Mutable Reference Restriction Fix is to just keep them immutable
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s; 
    println!("{}, {}", r1, r2); // hello, hello

// Mixing Immutable and Mutable Reference
    let mut s = String::from("hello");
    let r1 = &s;      // Immutable reference - OK
    let r2 = &s;      // Another immutable reference - OK
    // let r3 = &mut s;  // Mutable reference - ERROR!
    // println!("{}, {}, {}", r1, r2, r3);

    // Error: "cannot have a mutable reference if an immutable reference already exists"
    // Why?
    // - Immutable references don't expect the underlying value to change
    // - Having a mutable reference would violate this expectation

// Multiple immutable references are OK:
    // - Can have multiple immutable references
    // - OK because underlying data won't change
    let s = String::from("hello");
    let r1 = &s;// Immutable reference - OK
    let r2 = &s;// Another immutable reference - OK
    let r3 = &s;// Another immutable reference - OK  
    println!("{}, {}, {}", r1, r2, r3); // hello, hello, hello

// Reference Scope Rules
    // Reference scope starts when introduced, ends when last used
    let mut s = String::from("hello");
    let r1 = &s; // r1 scope start
    let r2 = &s; // r2 scope start
    println!("{}, {}",r1,r2);// r1 and r2 last used here
                             // r1 and r2 scope ends here
    // hello, hello

    let r3 = &mut s;  // YES, Works fine! r1 and r2 are out of scope
    // - r1 and r2 are out of scope by the time r3 is declared
    // - No conflict between immutable and mutable references
    println!("{}", r3);
    // hello

// Dangling References
    // What if a reference points to invalid data?
    // let reference_to_nothing = dangle();// No, error[E0106]: missing lifetime specifier  
    // Observe function

// Rule of References
    // 1. At any time , for a particular peice of data in a particular scope, 
    // we can have either 
    //     1.1. 1 mutable reference
    //     1.2. or any number of immutable References
    // 2. References must be valid && data they point to must be valid
                            
}

// References

    fn cal_len(s:String) -> usize {
        let len: usize = s.len();
        // len() returns the length of a String
        len
    }

    fn cal_len_1(s:String) -> (String,usize) {
        let len: usize = s.len();
        (s,len) // Return both string AND length
    }

    fn cal_len_2(s:&String) -> usize 
    // Borrowing: Passing references as function parameters
    // - You're borrowing the value
    // - Not taking ownership of it
    {  // Takes reference to String
        s.len()
    }  // s goes out of scope, but doesn't drop the value
    // because s doesn't own it

// Borrowing

// References are immutable by default

    // fn change(some_string: &String){
    //     some_string.push_str(", world !!!");
    //     // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    //     // ERROR! Cannot modify 
    // } // NO

// Mutable References
    fn change(some_string: &mut String){ // 3. Function takes mutable reference with `&mut`
        some_string.push_str(", world !!!");
    } // Yes

// Dangling References

    // No
    // fn dangle() -> &String {  // Returns reference to String
    //     let s = String::from("hello");  // s created in this scope
        
    //     &s  // Return reference to s
    // }  // s goes out of scope and is dropped
    // // Reference points to deallocated memory!

    // Error: "this function's return type contains a borrowed value, but there is no value for it to be borrowed from"

    // Why error?
    // - `s` is defined within function scope
    // - When function ends, `s` is dropped (deallocated)
    // - Reference would point to invalid memory
    // - Rust prevents this at compile time

    // Solution: Return the String directly (transfer ownership), not a reference. or use lifetime(ch10)

// Rust prevent us to do memory unsafe stuff