use std::any::Any;

trait M{}
struct S;
impl M for S{

}
fn main(){
    let s = S;
    let m :Box<dyn M> = Box::new(s);
    // check failed because it's wrapped with Box
    // let a = (&m as &(dyn Any)).downcast_ref::<S>() .unwrap();
    
    let mut s = S;
    // let m = &s as &(dyn Any);
    let a = (&mut s as &mut (dyn Any)).downcast_mut::<S>() .unwrap();


    let s = S;
    let a = (&s as &(dyn Any)).downcast_ref::<S>() .unwrap();
    let a = (&s as &(dyn Any)).downcast_ref::<dyn M>() .unwrap();

    let s = S;
    let m :Box<dyn M> = Box::new(s);
    let a = (&m as &(dyn Any)).downcast_ref::<S>() .unwrap();

    let box_any: Box<dyn Any> = Box::new(42_i32);
    
    // 尝试 downcast 到 Box<i32>
    if let Ok(box_i32) = box_any.downcast::<i32>() {
        println!("Got an i32: {}", *box_i32);
    } else {
        println!("Not an i32");
    }
}
