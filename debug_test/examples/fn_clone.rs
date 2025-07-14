fn main() {
    let s = |x: i32| x + 1;
    let m = s.clone();
    println!("{}", f(&m));
}

fn f(g: &dyn Fn(i32) -> i32) -> i32 {
    g(3)
}
