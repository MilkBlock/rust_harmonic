use std::{iter, vec};
fn main() {
    let r = iter::repeat("hello,").take(4);
    let t = iter::repeat("hello,");
    // println!("{:?}",);
    // println!("{:?}",);
    let s :Vec<i32> = iter::repeat(3).take(4).collect();
    let mut count = 1;
    let it = iter::from_fn(move ||{count+=1;if count<6{ Some(count)} else {None}});
    let v  = vec::Vec::from_iter(it.take(8));
    println!("{:?}",v);
    // // println!("{:?}",s);
    // println!("{:?}",s);
    let h = "Hello";
    let r = Some("hello");
    drop()

}
