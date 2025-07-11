use std::ops::Not;

use derive_more::Not;

#[derive(Not)]
struct MyInts(i32, i32);


struct PP{


}

fn main(){
    let a = 3.not();
    println!("{}",a);
}
