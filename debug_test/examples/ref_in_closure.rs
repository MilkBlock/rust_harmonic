struct S<T> {
    t: T,
}
impl<T> S<T> {
    fn new(t: T) -> Self {
        Self { t }
    }
}
fn main() {
    let a = 3;
    let c1 = || {
        let s = S::new(&a);
        s
    };

    let a = 3;
    let c2 = || &a;
}
