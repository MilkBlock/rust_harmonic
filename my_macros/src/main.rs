use macro_lib::{show_para, HelloMacro};

#[derive(HelloMacro)]
struct S;

#[derive(HelloMacro)]
struct S1;
fn main() {
    let s = S;
    s.hello();
}


trait M{

}

#[show_para]
fn show() -> (Root, M) {
    (1,2)
}