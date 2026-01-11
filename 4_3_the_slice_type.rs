fn main(){
// slices
    // - Let you reference a contiguous sequence of elements within a collection
    // - Instead of referencing the entire collection
    // - Like references, slices do NOT take ownership

// Problem Slice Solves
    // Goal: Write a function `first_word` that:
    // - Takes a reference to a String (don't want ownership)
    // - Returns the first word in the String

    // Initial solution: Return an index to the end of the first word
    // Problem with index based approach
    // Problem 1 : Return value not tied to the string
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // word = 5
    // h e l l o  
    // 0 1 2 3 4 5
    s.clear();  // s is now empty string
    
    // word is still 5, but string is empty!
    // Return value is out of sync with the string

    // Problem: Must manually keep return value in sync with string - extremely error-prone
    // PROBLEM 2: Returning second word becomes more complex

    // To return the second word:
    // - Would need to return a tuple
    // - Tuple with start index and end index
    // - Even more values to keep in sync with string

// Introducing String Slices
    // &s[start..end]
    // - Creates a reference to part of the string
    // - `start` is inclusive
    // - `end` is exclusive
    let s = String::from("hello world");
    // hello world
    // 012345678910
    let hello = &s[0..5]; // String slice from index 0 to 5 (exclusive)
    let world = &s[6..11]; // String slice from index 6 to 11 (exclusive)

// String Slice Shortcuts
    let slice = &s[0..5]; // Starting from beginning
    let slice = &s[..5]; // Starting from beginning // Can omit 0 
    
    let slice = &s[6..11]; // Continuing to end
    let slice = &s[6..]; // Continuing to end // Can omit end value
    
    let slice = &s[0..s.len()]; // Entire string
    let slice = &s[..];  // Entire string // Can omit both values

// Solving Problem with String Slices
    // Needs improvement
    let mut s: String = String:: from("hello world");
    let word: &str = first_word_using_slices(&s); // immutable borrow of s
    // s.clear();                                   // No, mutable borrow of s
    println!("{}", word);                        // immutable borrow still in use
    // BUT error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

// String literals are slices
    let s = "hello world";  // Type is &str
    // String literals:
    // - Are stored directly in the binary
    // - `s` is actually a string slice pointing to that location in binary
    // - Type is `&str`

// String Slices as parameters
// To make function work with both String and string literals:
    let mut s: String = String::from("hello world");
    let s2: &str = "hello world";

    let word: &str = first_word_using_slices_string_stringliterals(&s); 
    // let word: &str = first_word_using_slices_string_stringliterals(s2);

        let my_string = String::from("hello world");
    let word = first_word_using_slices_string_stringliterals(&my_string);  // Still works!
                                         // String reference coerced to slice
    
    let my_literal = "hello world";
    // let word = first_word_using_slices_string_stringliterals(my_literal);  // Also works!
                                        // String literal is already a slice

    // - this version is idiomatic and correct
    // - Demonstrates that the same function works for both:
    // ```rs
    // first_word_using_slices_string_stringliterals(&s);
    // first_word(s2);
    // ```
    // - Design functions to accept references (&str) instead of owned types (String) unless ownership is required.

// type notation
    // String => String type 
    // &str   => String Slice type
    
// Other Slices
    // Slices work on other collection types too:
    let a = [1,2,3,4,5];
    let slice = &a[0..2];

    // a => [i32; _]
    // slice => &[i32]

    // - Type is `&[i32]`
    // - `&` followed by square brackets with element type
    // - `&[i32]` because array stores signed 32-bit integers
}

// Problem Slice Solves
    
    // Goal: Write a function `first_word` that:
    // - Takes a reference to a String (don't want ownership)
    // - Returns the first word in the String
    
    // Thinking: fn first_word(s: &String) -> ?  // What's the return type?
    // Problem: Don't have a way to return part of the String
    
    // Initial solution: Return an index to the end of the first word
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // Convert string to array of bytes
        for (i, &item) in bytes.iter().enumerate(){
            if item == b' '{ // If we find a space
                return i; // Return the index
            }
        }
        s.len() // If no space found, entire string is one word
    }

// Introducing String Slices
// Solving Problem with String Slices
    fn first_word_using_slices(s: &String) -> &str{
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate(){
            if item == b' '{
                return &s[..i];
            }
        }
        &s[..]
    }

// String Slices as parameters
// To make function work with both String and string literals:
    fn first_word_using_slices_string_stringliterals(s: &str) -> &str {
        let bytes: &[u8] = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() 
        // You cannot annotate types inside pattern matching 
        {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }