use rudi::{Context, Singleton, Transient};

// Register `fn(cx) -> A { A }` as the constructor for `A`
#[derive(Debug)]
#[Transient]
struct A;

#[derive(Debug)]
struct B(A);

// Register `fn(cx) -> B { B::new(cx.resolve::<A>()) }` as the constructor for `B`
#[Transient]
impl B {
    #[di]
    fn new(a: A) -> B {
        println!("create B");
        B(a)
    }
}

// Register `fn(cx) -> C { C::B(cx.resolve::<B>()) }` as the constructor for `C`
#[allow(dead_code)]
#[Transient]
enum C {
    A(A),

    #[di]
    B(B),
}

// Register `fn(cx) -> () { Run(cx.resolve::<B>(), cx.resolve::<C>()) }` as the constructor for `()`
#[Singleton]
fn Run(b: B, c: C) {
    println!("create singleton");
    assert!(matches!(c, C::B(_)));
}

fn main() {
    // Automatically register all types and functions with the `#[Singleton]`, `#[Transient]` or `#[SingleOwner]` attribute.
    let mut cx = Context::auto_register();

    // Get an instance of `()` from the `Context`, which will call the `Run` function.
    // This is equivalent to `cx.resolve::<()>();`
    let a =cx.resolve::<()>();
    // let m = cx.resolve::<B>();
}