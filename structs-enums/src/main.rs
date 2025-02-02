#![allow(unused)]

// tag::define-struct[]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// end::define-struct[]

fn main() {
    println!("Hello, world!");
}
