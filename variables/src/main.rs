// tag::constants-a[]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// end::constants-a[]

fn main() {
    // tag::constants-b[]
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours."); // THREE_HOURS_IN_SECONDS = 10800
    // end::constants-b[]

    // tag::declaration[]
    let z;
    let y = 18;
    z = 93;
    println!("The value of y is: {y}"); // y = 18
    println!("The value of z is: {z}"); // z = 93
    // end::declaration[]

    // tag::mutability[]
    let mut x = 5;
    println!("The value of x is: {x}"); // x = 5
    x = 6;
    println!("The value of x is: {x}"); //x = 6
    // end::mutability[]

    // tag::shadowing[]
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x = 12
    }

    println!("The value of x is: {x}"); // x = 6
    // end::shadowing[]
}
