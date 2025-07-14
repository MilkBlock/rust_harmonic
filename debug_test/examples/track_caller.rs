use std::panic::Location;
const A: &'static Location<'static> = Location::caller();

#[track_caller]
fn print_caller(x: i32) {
    let caller = Location::caller();
    println!("caller is {:?}", caller);
}

trait M {
    #[track_caller]
    fn f() {
        print_caller(3);
    }
}
struct S;
impl M for S {
    fn f() {
        println!("{:?}", Location::caller())
    }
}

fn main() {
    S::f();
    // println!("{:?}",A);
}
