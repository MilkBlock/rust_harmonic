use std::{any::Any, collections::HashMap, hash::Hash, marker::PhantomData, ops::Deref};

struct A<T> {
    p: PhantomData<T>,
}

impl<T> Deref for A<T> {
    type Target = A<()>;
    fn deref(&self) -> &Self::Target {
        &A { p: PhantomData }
    }
}
fn main() {
    let mut a = HashMap::new();
    a.insert("a".to_string(), "b".to_string());
    let a = &mut a as &mut dyn Any;
    let b: &mut HashMap<String, String> = a.downcast_mut().unwrap();

    // a.insert("a", &"b".to_string());
    // a.get("m");
    let s = &*"st".to_string();
    println!("p");
}
