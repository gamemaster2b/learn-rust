/// Constants are declared using the 'const' keyword are immutable.<br/>
/// They can not be made mutable. Adding 'mut' will result in a compile time error.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours."); // THREE_HOURS_IN_SECONDS = 10800

    // Variables are declared using the 'let' keyword and immutable by default.
    // tag::declaration[]
    let y = 18;
    println!("The value of x is: {y}"); // y = 18
    // end::declaration[]

    // 'mut' is used to make a variable mutable.
    // tag::mutability[]
    let mut x = 5;
    println!("The value of x is: {x}"); // x = 5
    x = 6;
    println!("The value of x is: {x}"); //x = 6
    // end::mutability[]

    // Shadowing is the declaration of new variable using the name of a preexisting variable
    // The compiler identifies the second variable by the name until it leaves the scope or the second variable shadowed by another
    // tag::shadowing[]
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x = 12
    }

    println!("The value of x is: {x}"); // x = 6
    // end::shadowing[]
}
