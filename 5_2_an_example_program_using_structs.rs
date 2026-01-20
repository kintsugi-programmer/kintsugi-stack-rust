// An Example Program Using Structs
// Practical Example: Rectangle Area Calculator
fn main(){
    // Approach 1
    // Starting with seperate variables 
    let width = 30; let height = 50; 
    println!("Area = {} pixels", area(width,height));
    // Area = 1500 pixels

    // Approach 2
    // Refactoring with Tuples
    // Grouping related variables:
    // Improvements:
    // - Width and height grouped together in one variable
    // - Pass single variable instead of two
    // New Problem:
    // - Fields in tuple aren't named
    // - Not clear which is width or height (is first variable width or height?)
    let dimensions = (30,50);
    println!("Area = {} pixels", area_1(dimensions));
    // Area = 1500 pixels

    // Approach 3
    // Refactoring with Structs
    // Creating Rectangle struct:
    let rect = Rectangle{width:30, height: 50,};
    // Key Points:
    // - Function accepts reference to Rectangle (&Rectangle)
    // - **Reason for reference**: Want to use fields but not take ownership
    // - Pass reference from main: &rect
    // Benefits:
    // - Much more readable
    // - Type for rectangle with meaningful fields
    // - Clear what each value represents
    println!("Area = {} pixels", area_2(&rect));
    // Area = 1500 pixels

    // Derived Traits
    // Printing Debug Information
    // Attempting to print rectangle:
        // println!("{}",rect); // No
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // Error message: Rectangle doesn't implement the display trait
    // What is Display trait:
    // - Specifies how something should be printed
    // - Primitive types (integers) implement Display by default
    // - Only one way to print an integer
    // - Custom types (structs) must implement Display manually
    // Debug Formatting Syntax
    // Helpful hint suggests using:
       // println!("{:?}",rect); //error[E0277]: `Rectangle` doesn't implement `Debug`
    // New error: Rectangle doesn't implement Debug trait
    // What is Debug trait:
    // - Allows printing information useful to developers
    // - Must be explicitly added to struct
    // After Adding derive annotation:
        let rect = Rectangle_1 {
            width: 30, height: 50,
        };
    //  in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
        println!("{:?}",rect);
        // Rectangle_1 { width: 30, height: 50 }
        println!("{:#?}",rect);
        // Rectangle_1 {
        //     width: 30,
        //     height: 50,
        // }

    // Understanding Derive:
    // - Debug is a trait
    // - Adding derive allows compiler to provide basic implementation
    // - More about traits in Chapter 10


}

// Approach 1
// Starting with seperate variables 
fn area(width: u32, height: u32) -> u32 {
    width*height
}

// Approach 2
// Refactoring with Tuples
// Grouping related variables:
// Improvements:
// - Width and height grouped together in one variable
// - Pass single variable instead of two
// New Problem:
// - Fields in tuple aren't named
// - Not clear which is width or height (is first variable width or height?)
fn area_1(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Approach 3
// Refactoring with Structs
// Creating Rectangle struct:
struct Rectangle{
    width: u32,height: u32,
}
// Key Points:
// - Function accepts reference to Rectangle (&Rectangle)
// - **Reason for reference**: Want to use fields but not take ownership
// - Pass reference from main: &rect
// Benefits:
// - Much more readable
// - Type for rectangle with meaningful fields
// - Clear what each value represents
fn area_2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Derived Traits
// Adding Debug Trait
// Add derive annotation:
#[derive(Debug)]
struct Rectangle_1{
    width: u32,height: u32,
}