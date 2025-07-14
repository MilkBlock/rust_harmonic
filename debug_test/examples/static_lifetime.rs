trait A {
    fn a(&self);
}

struct S {}
impl A for S {
    fn a(&self) {
        println!("hello1")
    }
}
impl A for &S {
    fn a(&self) {
        println!("hello2")
    }
}
impl A for &&S {
    fn a(&self) {
        println!("hello3")
    }
}

fn main() {
    let s = S {};
    let s_ref = &s;
    (&&s_ref).a();
}
