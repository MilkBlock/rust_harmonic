static A:&[i32] = &[1,2,3];
static F:fn()->f32 = ||{
    32.1
};


trait A{
    fn f(&self){
        println!("A")
    }
}
trait B:A{
    fn f(&self){
        println!("B")
    }
}
struct S;
// impl A for S{

// }
// impl B for S{

// }
fn main(){
    // S::f();
    let mut s = S;
    let m = unsafe { &mut s as *mut S as *mut dyn B };
}

