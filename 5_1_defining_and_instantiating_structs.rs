// Structs
    // Helps to create new types
    // Allows to group related data together
    // Object Attributes in OOPs

// Defining a Struct
struct User{
    username: String, // not &str // convert using "".to_string() || String::from("") 
    email: String,
    sign_in_count: u64,
    active: bool   
}

// Advantages of Struct over Tuple
    // Grouping of different data
    // Naming the structure 
    // Naming the data inside

// Building Structs with Functions
fn build_user(
    email: String,
    username: String
) -> User
{
    // User{
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count:1,
    // }

    // Field Init Shorthand Syntax
    User {
        email, // When function arguments have the same name as struct fields, you can simplify
        username, // When function arguments have the same name as struct fields, you can simplify
        active: true,
        sign_in_count: 1
    }
}

// Tuple Structs
    // Structs without named fields
    // It gives Name just to entire tuple
    // Make tuple a different type than other tuples
    // Even with same field types, Color and Point are different type
    // A function expecting Point cannot accept Color
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // even thou these fields are same of both tuple struct, still they are not name, not even fun

// Unit-Like Structs
    // Structs without any fields exist but are covered in Chapter 10.

// String Ownership in Structs
    // username: String, // not &str // convert using "".to_string() || String::from("") 
    // Why String instead of string slices (&str):
        // - Fields OWN the string data
        // - Fields could reference borrowed data (string slices)
        // - Referencing borrowed data requires using lifetimes
        // - Lifetimes are covered in Chapter 10

fn main(){
    // Creating Struct Instances
    let user1 = User{
        email: "kintsugiprogrammer@gmail.com".to_string(),
        username: "kintsugi-programmer".to_string(),
        active: true,
        sign_in_count: 1,
    }; // Attributes can be specified in any order when creating an instance.

    // Accessing Struct Fields
    let name = user1.username ; // use dot notation

    // Modifying Struct Fields ( make entire struct mutable + use dot notation to change values )
    // user1.username = "bali-king".to_string(); // No, error[E0594]: cannot assign to `user1.username`, as `user1` is not declared as mutable
    let mut user1 = User{
        email: "kintsugiprogrammer@gmail.com".to_string(),
        username: "kintsugi-programmer".to_string(),
        active: true,
        sign_in_count: 1,
    }; // mutable
    user1.username = "bali-king".to_string(); // Yes
    // Critical Rule: You cannot make just one field of the struct mutable - the entire struct must be mutable.

    // Building Structs with Functions
    user1 = build_user(String::from("kintsugiprogrammer@gmail.com"),String::from("kintsugi-programmer"));

    // Creating Instances from Other Instances
    let user2 = User {
        email: String::from("telemetrytradeai@gmail.com"),
        username: String::from("TelemetryTradeAI"),
        ..user1 // Use **..user1** syntax to pull remaining values, Remaining fields (active and sign_in_count) come from user1
    };

    let tup1 = Color(120,0,0);
    let tup2 = Point(-10,2,3);

    // String Ownership in Structs
        // username: String, // not &str // convert using "".to_string() || String::from("") 

}

