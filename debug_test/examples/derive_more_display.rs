use derive_more::Display;

#[derive(Display)]
#[display("hello {} {}", a, b)]
struct A {
    a: i32,
    b: B,
}
#[derive(Display)]
struct B {
    a: i32,
}
fn main() {
    let a = A {
        a: 3,
        b: B { a: 5 },
    };
    println!("{}", a);
    HashMap
}
