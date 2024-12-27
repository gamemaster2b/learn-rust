/// Constants are declared using the 'const' keyword are immutable.<br/>
/// They can not be made mutable. Adding 'mut' will result in a compile time error.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    // Variables are declared using the 'let' keyword and immutable by default.
    let y = 18;
    println!("The value of x is: {y}");

    // 'mut' is used to make a variable mutable.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing is the declaration of new variable using the name of a preexisting variable
    // The compiler identifies the second variable by the name until it leaves the scope or the second variable shadowed by another
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
