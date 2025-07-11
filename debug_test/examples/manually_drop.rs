use std::mem::ManuallyDrop;

fn main(){
    let mut x = ManuallyDrop::new(Box::new(42));
    unsafe {
        ManuallyDrop::drop(&mut x);
    }
    let mut y = x; // Undefined behavior!
    unsafe {
        ManuallyDrop::drop(&mut y);
    }
}