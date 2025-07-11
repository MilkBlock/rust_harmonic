use std::{borrow::Borrow, rc::Rc};

struct B;
struct A;

/// borrow is used as Comparable And Simple AsRef
impl Borrow<B> for A{
    fn borrow(&self) -> &B {
        &B
    }
}
impl AsRef<B> for A{
    fn as_ref(&self) -> &B {
        &B
    }
}

fn hash<T:Borrow<str>>(t:T){
    let a:&str = t.borrow();
    println!("{}",a);
}
fn main(){
    let a = A;
    let v:Vec<&B> = vec![a.borrow()];
    drop(a);
    let v = vec![3];
    v.iter().for_each(|x|{println!("{:?}",x);});
}