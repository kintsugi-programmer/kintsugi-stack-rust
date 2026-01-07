## 1. Getting Started

**SERIES OVERVIEW**

- This is a chapter-by-chapter series covering the Rust Programming Language book (also known as "the book")

### 1.1. Installation

**INSTALLING RUST**

**For Linux and macOS:**

- Installation command to copy and paste into terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- After running this command, you should see text confirming "Rust is installed now"
- Important: After installation, reload your terminal so the paths are updated

**For Windows:**

- Installation steps are different but fairly simple
- Refer to the Rust book for specific Windows installation instructions

**INSTALLING RUST LANGUAGE SERVER**

**Purpose of Language Server:**

- Provides code completion
- Enables go-to-definition functionality
- Offers refactoring abilities

**For Visual Studio Code Users:**

- Open the Extensions tab
- Search for "rust"
- You'll see two main options:
  - First result: Official Rust programming language extension (being deprecated - don't install this one)
  - **Second result: rust-analyzer (install this one)**
- After installation, you may get a pop-up saying "the language server for Rust is not yet installed" - go ahead and download it
- You might see an error "rust analyzer failed to discover workspace" - this is normal when not using Cargo, can be ignored

### 1.2. Hello, World!

**WRITING YOUR FIRST RUST PROGRAM**

**Setup:**

- Create a new directory called `hello_world`
- Navigate into it using `cd hello_world`
- Open it in VS Code

**Creating the File:**

- Create a new file named `main.rs`
- The `.rs` extension is for Rust files

**The Code:**

```rust
fn main() {
    println!("Let's get rusty");
}
```

**Code Breakdown:**

- `fn main()` - This is the main function that executes when your program starts
- `println!` - This is a macro (you'll learn about macros later)
- The text in quotes can be anything you want (traditionally "Hello, World!" but this example uses "Let's get rusty")
- Don't forget the semicolon at the end

**Compiling and Running:**

- Save your work
- Go to terminal
- Compile using: `rustc main.rs`
- This creates a new executable file called `main`
- Run the executable: `./main` (on Linux/macOS) or `main.exe` (on Windows)
- Output should display your message

### 1.3. Hello, Cargo!

**INTRODUCTION TO CARGO**

**What is Cargo:**

- Cargo is Rust's build system and package manager
- Comes built-in with Rust installation
- Handles projects with multiple files and dependencies
- This is a pain point with other low-level programming languages, but Rust has it built-in

**Verifying Cargo Installation:**

```bash
cargo --version
```

- Should show something like: `cargo 1.49.0` (version numbers will vary)

**CREATING A PROJECT WITH CARGO**

**Creating a New Project:**

```bash
cargo new hello_cargo
```

- This command creates a new package
- Cargo automatically initializes a Git repository for the project
- You can use a different source control system by specifying it during creation

**Project Structure:**

**1. Cargo.toml File:**

- This is the configuration file for your package
- Similar to `package.json` in the web development world
- Contains:
  - Package name
  - Version number
  - Edition information
  - Dependencies section (currently empty, but any dependencies would be listed here)

**2. .gitignore File:**

- Created because Cargo automatically initializes a Git repo

**3. src/ Directory:**

- Contains your actual code
- Includes a `main.rs` file automatically generated with a hello world program

**BUILDING AND RUNNING CARGO PROJECTS**

**Building the Project:**

```bash
cd hello_cargo
cargo build
```

**What Happens:**

- Creates a new file called `Cargo.lock`
  - Specifies exact dependencies
  - Currently sparse because there are no dependencies
- Creates a `target/` directory
  - Contains a `debug/` directory
  - Includes the actual executable
  - Plus other build artifacts

**Running the Project:**

```bash
cargo run
```

- This command both builds and runs your program
- Output: Displays "Hello, World!" (or whatever message is in your code)

**OTHER USEFUL CARGO COMMANDS**

**Checking for Errors:**

```bash
cargo check
```

- Checks your program for errors without producing an executable
- Much faster than building because it doesn't create the executable
- Useful during development

**Viewing All Commands:**

```bash
cargo help
```

- Shows all available Cargo commands

**CHAPTER COMPLETION**

- Chapter 1 of the Rust book is now complete
- Next chapter will cover more advanced topics

## 2. Programming a Guessing Game

**Project Overview**
- Building a fully functional AAA-rated guessing game
- Includes additional features not covered in the original book
- Previous chapter covered Hello World program (Chapter 1)

**Setting Up the Project**

**Initial Project Creation**
- Open terminal
- Run command: `cargo new guessing_game`
- Navigate into directory: `cd guessing_game`
- Open project in VS Code
- Standard Hello World program is generated by default

**Running the Program**
- Method 1: Type `cargo run` in terminal
- Method 2: Use VS Code button (if language server installed)
  - Language server provides a neat button to run programs directly in VS Code

**Building the Basic Game Structure**

**Step 1: Prompting User for Input**
- Change the print statement from "Hello World" to prompt users to input a number
- Use print statement to display the prompt message

**Step 2: Creating Variables to Store Input**
- Use the `let` keyword to create variables in Rust
- Create a variable named `guess`
- Set it equal to a String

**Understanding String Type**
- **String** is a type in Rust's standard library
- It's a UTF-8 encoded growable string
- More details on strings covered in later chapters

**String Creation Syntax**
- Use `String::new()` syntax
- **new** is a function on the String type
- It's an **associative function** (called static methods in other languages)
- **new** returns an empty string that can be used

**Rust's Type Inference**
- Rust is smart enough to realize variable `guess` is of type String
- No need for explicit type annotation
- Rust knows the `new` function returns a String

**Variable Mutability**
- Variables in Rust are **immutable by default**
- Need to change `guess` to capture user input
- Use the **mut** keyword to make variables mutable
- Syntax: `let mut guess = String::new();`

**Step 3: Bringing IO Library into Scope**
- Need the **io** library to capture user input
- Bring library into scope using **use** statement
- Syntax: `use std::io;`
  - **std** = standard library
  - **io** = input/output library

**Step 4: Capturing User Input**
- Use the io library: `io::stdin()`
- **stdin** function gives a handle to standard input of current process
- **read_line** takes user's input and appends it to specified buffer
- In this case, the buffer is the `guess` string

**Understanding read_line Function**
- Takes a **mutable reference** to String
- References covered in later chapters
- Essentially: takes a reference to String, modifies it without taking ownership
- Syntax: Pass in `&mut guess`

**Understanding Result Type**
- **read_line** returns a **Result** type
- Result is an **enumeration** (enum)
- Can return two variants:
  - **Ok**: holds the return value
  - **Err**: holds an error object
- Forces developers to handle error cases

**Error Handling with expect**
- Use the **expect** function for error handling
- If **Ok** variant returned: expect returns the value contained in Ok
- If **Err** variant returned: program panics with provided message
- Example message: "failed to read line"
- In this instance, errors are likely system errors

**Step 5: Printing User Input**
- Use `println!` statement
- Use **curly brackets** `{}` as placeholders for variables
- Syntax: `println!("You guessed: {}", guess);`
- Curly brackets are replaced with the guest string value

**Testing the Program So Far**
- Run command: `cargo run`
- Program prompts for input
- Type in a number (example: 22)
- Press Enter
- Output shows: "You guessed: 22"

**Current functionality achieved:**
- Prompted user for input
- Captured the input
- Printed it back out

**Adding Random Number Generation**

**Why External Dependency Needed**
- Rust's standard library doesn't include random number generation
- Need to add a new dependency

**Adding the rand Dependency**
- Open `Cargo.toml` file
- Under **dependencies** section, add: `rand` with version number
- Save the file
- Run `cargo build`
- This downloads **rand** and its dependencies

**Using the rand Dependency**

**Bringing rand into Scope**
- Go back to `main.rs`
- Add another **use** statement
- Bring the **Rng trait** into scope
- Syntax: `use rand::Rng;`

**Understanding Rng Trait**
- **Rng trait** defines methods that random number generators use
- Must bring it into scope to use random number functionality
- Traits covered in detail in later chapters

**Generating Random Number**
- Create new variable using `let` keyword
- Variable name: `secret_number`
- Syntax: `let secret_number = rand::thread_rng().gen_range(1..101);`

**Understanding Random Number Functions**
- **thread_rng**: associative function that gives a random number generator
- **gen_range**: method that produces a random number within specified range
- Range: 1 to 101 (high value is exclusive, so actual range is 1-100)

**Printing the Secret Number**
- Use `println!` with curly brackets
- Syntax: `println!("The secret number is: {}", secret_number);`
- This is for testing purposes

**Testing Random Number Generation**
- Run: `cargo run`
- First run shows: 72
- Run again: shows different number (81)
- Confirms random number generation works

**Comparing User Guess with Secret Number**

**Bringing Ordering into Scope**
- Need to bring **Ordering** into scope
- **Ordering** is an enum that represents result of comparing two things
- Three variants:
  - **Less**: first value is less than second
  - **Greater**: first value is greater than second
  - **Equal**: values are equal

**Using the cmp Method**
- Scroll to end of program
- Use `guess.cmp(&secret_number)`
- **cmp** takes a reference to the thing being compared (secret_number)
- **cmp** returns an **Ordering** enum (Less, Greater, or Equal)

**Handling Different Outcomes with Match Expression**
- Need to print different strings depending on Ordering variant returned
- Rust uses **match expressions** for this

**Creating Match Expression**
- Put the word **match** in front of the statement
- Add curly brackets after the statement
- Match all possible return values:
  - `Ordering::Less => println!("Too small!")`
  - `Ordering::Greater => println!("Too big!")`
  - `Ordering::Equal => println!("You win!")`

**Type Mismatch Error**
- Red squiggly lines appear after saving
- Error: can't compare type String to type integer
- **secret_number** is an integer
- **guess** is a String
- Need to convert String to integer

**Converting String to Integer**

**Shadowing Variables**
- Create a new `guess` variable right after getting user input
- Take the first `guess` variable (String) and convert it

**Conversion Process**
- Call **trim** on the string
  - **trim** removes whitespace at beginning or end of line
- Call **parse** function
  - **parse** converts string into another type
  - Parse doesn't know what type to convert to automatically

**Type Annotation**
- Annotate the variable to give parse a hint
- Syntax: `let guess: u32`
- **u32** = unsigned 32-bit integer

**Handling Parse Errors**
- **parse** function can fail
- Returns a **Result** type (parsed value or error)
- Use **expect** function with error message
- If parse fails, user probably didn't type a number
- Message: "Please type a number!"

**Result of Conversion**
- Red squiggly lines disappear
- Comparison logic at end of program is now valid

**Testing Comparison Logic**
- Run: `cargo run`
- Example 1: Secret is 47, guess 46 → "Too small!"
- Example 2: Secret is 75, guess 76 → "Too big!"
- Example 3: Secret is 50, guess 50 → "You win!"
- Comparison logic works correctly

**About Shadowing**
- Declared `guess` as String type
- Later redeclared `guess` as u32 type
- This is called **shadowing**
- Often used to convert a variable from one type to another while preserving same name
- More details covered in Chapter 3

**Adding Loop for Multiple Guesses**

**Current Problem**
- If user guesses wrong, program tells them too small/too big and quits
- Want users to keep guessing until they get correct number

**Implementing Loop**
- Add **loop** keyword before prompting user for input
- Put everything underneath inside the loop
- Syntax: `loop { ... }`

**Testing the Loop**
- Run: `cargo run`
- Secret number: 70
- Guess 50 → "Too small!" → can guess again
- Guess 70 → "You win!" → but game continues
- Can type 70 again → says won again
- Loop never ends because nothing tells it to stop

**Problem with Invalid Input**
- Type "quit" → program panics (not a number)
- Any non-number input causes panic and program exits

**Adding Break Statement**
- Go to match statement
- Modify the Equal arm
- Not only print "You win!" but also **break** out of loop
- Syntax: `Ordering::Equal => { println!("You win!"); break; }`

**Testing Break Statement**
- Run: `cargo run`
- Secret: 43
- Guess 42 → "Too small!"
- Guess 43 → "You win!" → program exits as expected

**Improving Invalid Input Handling**

**Current Behavior**
- Run program and type random string → panic and exit
- Not ideal user experience

**Desired Behavior**
- Keep prompting user to input a number until it's valid
- Don't exit program on invalid input

**Modifying Error Handling**
- Modify the line that panics if parse fails
- **parse** returns Result enum (Ok or Err)
- Can use **match expression** to handle both cases

**Implementing Match for Parse Result**
- Include **match** keyword at beginning of line
- Instead of calling expect, use curly brackets
- Handle both variants:
  - **Ok(num)**: wraps output (u32 integer) → return it
  - **Err(_)**: catch-all value using underscore → continue to next loop iteration

**Match Expression Structure**
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

**Understanding the Catch-All Pattern**
- Underscore `_` is a catch-all value
- Means: no matter what error we get, execute same action
- Action: **continue** to next iteration of loop

**Testing Invalid Input Handling**
- Run program
- Give invalid input (random string)
- Program keeps asking to enter guess
- Only accepts valid numbers
- Program is now fully functional

**Adding Color Enhancement**

**Purpose**
- Make game better with colored text
- Not in original book
- Too small/too big → red text
- Correct guess → green text

**Adding colored Dependency**
- Go back to `Cargo.toml` file
- Add new dependency: `colored`
- Save file
- Run `cargo build`

**Using the colored Library**
- Go back to `main.rs`
- Scroll to top
- Bring color library into scope
- Syntax: `use colored::*;`

**Modifying Print Statements**
- Scroll down to comparison logic
- Use curly brackets with color methods

**For Red Text (Too Small/Too Big)**
- Call `.red()` on text to be displayed in red
- Example: `println!("Too small! {}", "".red());`
- Modify print statements for both "Too small!" and "Too big!"

**For Green Text (You Win)**
- Call `.green()` on text
- Example: `println!("{}", "You win!".green());`

**Testing Colored Output**
- Run: `cargo run`
- Secret: 39
- Guess 38 → red text for "Too small!"
- Guess 39 → green text for "You win!"

**Final Program Statistics**
- Under 40 lines of code
- Fully functional guessing game
- Includes color enhancement

**Concepts Learned in Chapter 2**

**Variables**
- Creating variables with `let` keyword
- Mutable vs immutable variables
- Shadowing variables

**Match Statement**
- Handling multiple possible outcomes
- Matching enum variants
- Using catch-all patterns with underscore

**Methods**
- Calling methods on types
- Examples: trim(), parse(), cmp()

**Associative Functions**
- Functions associated with types
- Called using `::` syntax
- Examples: String::new(), rand::thread_rng()

**Using External Crates**
- Adding dependencies in Cargo.toml
- Bringing libraries into scope with `use`
- Examples: rand, colored

**Error Handling**
- Result type and its variants (Ok, Err)
- Using expect() for simple error handling
- Using match expressions for custom error handling

**Loops**
- Creating infinite loops with `loop`
- Breaking out of loops with `break`
- Continuing to next iteration with `continue`

**Type System**
- Type inference
- Type annotations
- Type conversion with parse()

**Call to Action**
- Give video a like if enjoyed
- Subscribe for more Rust content
- Hit notification bell for new video alerts
- Next video will cover next chapter

**Program Structure Summary**

**Complete Flow:**
- Set up Cargo project
- Import necessary libraries (io, rand, colored)
- Generate random secret number (1-100)
- Loop starts:
  - Prompt user for guess
  - Read user input as String
  - Convert String to u32 (with error handling)
  - Compare guess with secret number
  - Print result in appropriate color:
    - Red if too small or too big
    - Green if correct
  - Break loop if correct
  - Continue loop if incorrect or invalid input
- End program when correct guess is made

## 3. Common Programming Concepts

Topics Covered: Variables, Basic Types, Functions, Control Flow, Comments

### 3.1. Variables and Mutability

VARIABLES AND MUTABILITY

**Variables are Immutable by Default**

- In Rust, when you create a variable, it cannot be changed unless explicitly made mutable
- This is a core safety feature of Rust

Example of Immutable Variable Error:

```rust
let x = 5;
println!("{}", x);
x = 6;  // ERROR: cannot assign twice to immutable variable
println!("{}", x);
```

Error message: "cannot assign twice to an immutable variable"


**Making Variables Mutable**

- To allow a variable to be changed, use the `mut` keyword after `let`
- This explicitly declares that the variable can be reassigned

Correct syntax:

```rust
let mut x = 5;
println!("{}", x);  // prints 5
x = 6;
println!("{}", x);  // prints 6
```

Running this with `cargo run` will print:
- First line: 5
- Second line: 6


**Constants**

- Constants are values that can NEVER change
- Created using the `const` keyword instead of `let`
- Constants have several requirements and differences from variables

Syntax example:

```rust
const SUBSCRIBER_COUNT: u32 = 100_000;
```

**Key Differences Between Constants and Variables:**

1. **Cannot be mutable**
   - You cannot use `mut` with constants
   - Typing `mut` before a constant gives error: "const globals cannot be mutable"
   - If you declare a constant, you can be absolutely sure it will never be mutated

2. **Must have type annotation**
   - Variables can have inferred types (like `x` being inferred as i32)
   - Constants MUST have their type explicitly specified
   - In the example: `u32` means unsigned 32-bit integer

3. **Can only be set to constant expressions**
   - Variables can be set to return values of functions
   - Constants CANNOT be set to function return values
   - Constants cannot be set to any value computed at runtime
   - Must be compile-time constant values

4. **Naming convention**
   - Common practice: all uppercase letters
   - Use underscores where there would be spaces
   - Example: `SUBSCRIBER_COUNT`

5. **Numeric literal readability**
   - Large numbers can be hard to read (like 100000)
   - Rust allows underscores in numeric literals for readability
   - Example: `100_000` is much easier to read than `100000`


**Shadowing**

- Shadowing allows you to create a new variable using an existing variable name
- The first variable is "shadowed" by the second variable
- This is different from mutability

Example:

```rust
let x = 5;
let x = 6;  // This shadows the first x
```

**Two Major Advantages of Shadowing:**

1. **Preserves Immutability**
   - The first `x` is still immutable
   - The second `x` is also immutable
   - No `mut` keyword needed
   - Both variables remain immutable despite the reassignment

2. **Can Change Types**
   - You can change the type when shadowing
   - This is NOT possible with mutable variables

Example with type change:

```rust
let x = 5;           // x is an integer
let x = "6";         // x is now a string
println!("{}", x);   // prints "5"
println!("{}", x);   // prints "6" (as string)
```

Running this program:
- First x prints: 5 (integer)
- Second x prints: "6" (string)

### 3.2. Data Types

DATA TYPES

**Overview**

Rust has two main categories of data types:
1. Scalar data types - represent a single value
2. Compound data types - represent a group of values


SCALAR DATA TYPES

Rust has **four main scalar data types:**
1. Integers
2. Floating-point numbers
3. Booleans
4. Characters


**1. INTEGERS**

- Numbers without a fractional component (whole numbers)
- Every integer has a length and can be signed or unsigned

**Signed vs Unsigned:**
- Signed integers: can be positive OR negative numbers
- Unsigned integers: can ONLY be positive numbers

**Integer Sizes:**
- 8 bits
- 16 bits
- 32 bits
- 64 bits
- 128 bits
- `isize`/`usize`: depends on architecture (usually 64 or 32 bit)

**Default Integer Type:**
- Rust defaults to **signed 32-bit integer** (i32)

**Ways to Represent Integers:**
- Decimal (standard)
- Hexadecimal (hex)
- Octal
- Binary
- Byte

**Integer Overflow:**

Example with 8-bit unsigned integer:

```rust
let x: u8 = 255;  // Max value for u8 is 255
```

What happens if you try to set it to 256 or higher:

- **In debug builds:** Rust will panic (crash with error)
- **In release builds:** Rust performs two's complement wrapping
  - Values greater than maximum wrap around to minimum
  - 256 becomes 0
  - 257 becomes 1
  - And so on

**Language Server Warning:**
- If you have a language server running and try to set `x` to 256
- You'll get an error warning you of the overflow before compilation


**2. FLOATING-POINT NUMBERS**

- Numbers with decimal points
- Support basic numeric operations

**Default Type:**
- 64-bit double precision floating point number (f64)

**Available Operations:**

```rust
// Addition
let sum = 5.0 + 10.0;

// Subtraction
let difference = 95.5 - 4.3;

// Multiplication
let product = 4.0 * 30.0;

// Division
let quotient = 56.7 / 32.2;

// Remainder
let remainder = 43.0 % 5.0;
```


**3. BOOLEANS**

- Represent the values true or false
- Type annotation: `bool`

```rust
let t = true;
let f: bool = false;
```


**4. CHARACTERS**

- Represents a Unicode character
- Written with **single quotes** (not double quotes)
- More details covered in Chapter 8

```rust
let c = 'z';
let heart = '❤';
```


COMPOUND TYPES

**Overview:**
- Types that represent a group of values
- Two main compound types: tuples and arrays


**1. TUPLES**

- Fixed-size collection of related data
- Can contain different types
- Written as comma-separated list inside parentheses

**Creating a Tuple:**

```rust
let tup = ("Let's Get Rusty", 100_000);
```

This tuple contains:
- First value: String representing YouTube channel name
- Second value: Integer representing subscriber count

**Type Annotation (automatic):**
- Types are automatically annotated
- Example type: `(&str, i32)`

**Two Ways to Get Values from Tuples:**

**Method 1: Destructuring**

```rust
let tup = ("Let's Get Rusty", 100_000);
let (channel, sub_count) = tup;
```

How this works:
- Creates new variables inside parentheses: `channel` and `sub_count`
- Sets equal to the tuple
- First variable gets first value from tuple
- Second variable gets second value from tuple

**Method 2: Dot Notation**

```rust
let tup = ("Let's Get Rusty", 100_000);
let sub_count = tup.1;  // Accesses second element
```

Important note:
- Tuples (like arrays) start at index 0
- `tup.0` would access the first element
- `tup.1` accesses the second element


**2. ARRAYS**

- Fixed-length collection in Rust
- For dynamic size, use a **vector** (covered in Chapter 8)

**Declaring Arrays:**

Syntax 1 - Comma-separated list in brackets:

```rust
let arr = [1, 2, 3];
```

Syntax 2 - Repeated value:

```rust
let arr = [0; 8];  // Creates array with 8 values, all set to 0
```

This syntax means: "create an array with 8 values, all initialized to 0"

**Accessing Array Values:**

Standard bracket syntax:

```rust
let arr = [1, 2, 3];
let first = arr[0];   // Gets first element (1)
let second = arr[1];  // Gets second element (2)
```

**Array Bounds Checking:**

Example of invalid access:

```rust
let arr = [1, 2, 3];
let x = arr[3];  // Index 3 does not exist (only 0, 1, 2 are valid)
```

Running `cargo run`:
- Results in: **out of bounds exception**
- Rust prevents memory-unsafe operations
- Rust can do this because it knows the array size at compile time
- This is a safety feature preventing undefined behavior

### 3.3. Functions

FUNCTIONS

**Basic Function Declaration**

- Functions declared using `fn` keyword
- Can be defined anywhere (even after main function)

Example:

```rust
fn main() {
    print_another_function();
}

fn print_another_function() {
    println!("Another function!");
}
```

**Naming Convention:**
- Rust uses **snake_case** for function names
- All lowercase letters
- Use underscores where there are spaces
- Example: `print_another_function`

**Calling Functions:**
- Specify function name followed by parentheses
- Example: `print_another_function();`
- Running `cargo run` will execute the function


**Function Parameters**

- Specify parameter name and type inside parentheses
- Must include type annotation for each parameter

Single parameter:

```rust
fn my_function(x: i32) {
    println!("The value is: {}", x);
}
```

Multiple parameters:

```rust
fn my_function(x: i32, y: i32) {
    println!("x: {}, y: {}", x, y);
}
```

- Separate multiple parameters with commas
- Each parameter requires type annotation

**Calling with Arguments:**

```rust
fn main() {
    my_function(11, 22);
}

fn my_function(x: i32, y: i32) {
    println!("x: {}, y: {}", x, y);
}
```

Running this prints: `x: 11, y: 22`


**Statements vs Expressions**

**Important Concept in Rust:**

**Statements:**
- Perform some action
- Do NOT return a value
- Example: `println!` statements

**Expressions:**
- Return a value
- Can be used on the right side of assignments

Example:

```rust
let sum = x + y;
```

- `x + y` is an **expression** because it adds and returns a value
- `let sum = ...;` is a **statement**


**Returning Values from Functions**

**Two Ways to Return Values:**

**Method 1: Using the `return` keyword**

```rust
fn add_numbers(x: i32, y: i32) -> i32 {
    let sum = x + y;
    return sum;
}
```

Key points:
- Specify return type after parentheses: `-> i32`
- Use arrow (`->`) followed by type
- Use `return` keyword to return the value

**Method 2: Implicit Return (Last Expression)**

```rust
fn add_numbers(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum  // No semicolon!
}
```

Important rules:
- The last expression in a function is **implicitly returned**
- **Omit the semicolon** on the last expression
- This is idiomatic Rust style

**Further Simplification:**

```rust
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // Returns result directly, no semicolon
}
```

**Using Return Values:**

```rust
fn main() {
    let sum = add_numbers(11, 22);
    println!("Sum: {}", sum);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}
```

Running this program prints:
- `x: 11, y: 22`
- `Sum: 33`

### 3.4. Control Flow

CONTROL FLOW

**IF STATEMENTS**

Basic syntax (familiar if you've programmed before):

```rust
let number = 5;

if number < 10 {
    println!("Number is less than 10");
} else if number < 20 {
    println!("Number is less than 20");
} else {
    println!("Number is 20 or greater");
}
```

Components:
- `if` block with a condition
- Code executes if condition is true
- Optional `else if` blocks
- Optional final `else` block

**Important Difference in Rust:**
- The condition MUST be explicitly a boolean
- Cannot evaluate non-boolean values

**This will cause an error:**

```rust
let number = 5;
if number {  // ERROR!
    println!("Number is something");
}
```

Error: trying to evaluate an integer, but need a `bool`

**Must explicitly check:**

```rust
if number != 0 {  // Returns boolean
    println!("Number is not zero");
}
```


**If-Else in Let Statements**

You can use if-else expressions inside let statements:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

How this works:
- Variable `condition` is set to `true`
- Variable `number` is set to the result of the if-else expression
- If condition is true, number becomes 5
- If condition is false, number becomes 6


LOOPS

**Rust has Three Types of Loops:**
1. `loop` - infinite loop
2. `while` - conditional loop
3. `for` - iterator loop


**1. LOOP - Basic Infinite Loop**

Created using the `loop` keyword:

```rust
loop {
    println!("Again!");
}
```

Behavior:
- Executes code inside loop continuously
- Runs until `break` is called
- Without `break`, runs forever
- Press `Ctrl+C` to exit the program

**Adding Break Statement:**

```rust
loop {
    println!("Again!");
    break;
}
```

Now the loop only executes once, then exits.


**Returning Values from Loops**

Loops can return values:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter;  // Returns counter value
    }
};

println!("Result: {}", result);  // Prints: Result: 10
```

How this works:
- Create mutable variable `counter` set to 0
- Each loop iteration increments counter
- If statement checks when counter reaches 10
- When it does, `break counter` returns the counter value
- The loop's return value is assigned to `result`
- Must add semicolon after the loop
- Running prints: 10


**2. WHILE LOOP - Conditional Loop**

Executes as long as a condition is true:

```rust
let mut number = 3;

while number != 0 {
    println!("{}", number);
    number -= 1;
}

println!("Liftoff!");
```

How this works:
- Variable `number` starts at 3
- While number does not equal 0, execute loop body
- Loop body prints the number and decrements it
- When number reaches 0, loop ends
- Final print statement executes

Running this program outputs:
```
3
2
1
Liftoff!
```


**3. FOR LOOP - Iterator Loop**

Useful for looping through collections:

**Looping Through Arrays:**

```rust
let arr = [10, 20, 30, 40, 50];

for element in arr {
    println!("{}", element);
}
```

How this works:
- Have a collection (array) of integers
- Syntax: `for element in arr`
- `arr` gives us an iterator for the array
- For every element in the array, take that element and print it

**Looping Through Ranges:**

```rust
for number in 1..4 {
    println!("{}", number);
}
```

Key points:
- Uses range syntax: `1..4`
- Range is a type from the standard library
- Represents a sequence of numbers
- The last number is **exclusive** (not included)
- `1..4` creates sequence: 1, 2, 3 (not 4)
- Parentheses are optional around the range

Running this program outputs:
```
10
20
30
40
50
1
2
3
```

The for loop:
- First prints all values in the array
- Then prints all values in the range

### 3.5. Comments

COMMENTS

**Two Basic Types of Comments in Rust:**

**1. Single-Line Comments**

```rust
// This is a single-line comment
// Use two forward slashes
```

**2. Block Comments**

```rust
/*
This is a block comment
It can span multiple lines
Start with forward slash and asterisk
End with asterisk and forward slash
*/
```

Syntax:
- Start: `/*`
- End: `*/`

**Document Comments**
- There are also document comments
- Will be learned at another time
- Used for documentation generation

CHAPTER 3 COMPLETE

**Topics Covered:**
- Variables and immutability
- Constants
- Shadowing
- Basic data types (scalar and compound)
- Functions and return values
- Control flow (if statements and loops)
- Comments

**Next Steps:**
- Continue to Chapter 4
- Practice the concepts learned
- Build projects using these fundamentals