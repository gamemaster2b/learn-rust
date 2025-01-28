// tag::constants-a[]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// end::constants-a[]

fn main() {
    // tag::constants-b[]
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours."); // outPut: There are 10800 seconds in 3 hours.
    // end::constants-b[]

    // tag::declaration[]
    let z;
    let y = 18;
    z = 93;
    println!("The value of y is: {y}"); // outPut: The value of y is: 18
    println!("The value of z is: {z}"); // outPut: The value of y is: 93
    // end::declaration[]

    // tag::mutability[]
    let mut x = 5;
    println!("The value of x is: {x}"); // outPut: The value of y is: 5
    x = 6;
    println!("The value of x is: {x}"); // outPut: The value of y is: 6
    // end::mutability[]

    // tag::shadowing[]
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // outPut: The value of x in the inner scope is: 12
    }

    println!("The value of x is: {x}"); // outPut: The value of y is: 6
    // end::shadowing[]
}
