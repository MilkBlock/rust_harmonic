
use std::collections::HashMap;

use polonius_the_crab::{polonius, polonius_return};


// fn get_or_insert<'a,'b>(map: &'a mut Vec<u32>) -> Option<&u32> where 'a:'b{
//     let v = if let Some(v) = map.get(22){
//         Some(v)
//     }else{
//         map.push(3);
//         Some(&map[22])
//     };
//     v
// }
fn get(x:&mut Vec<u32>) {
    let v = if let Some(v) = x.get(22){
        Some(v)
    }else{
        x.push(3);
        Some(&x[22])
    };
    println!("{:?}",v);
}
fn main() {
    let mut x=  vec![2,3,4];
    let x = &mut x;
    for i in x{
        println!("{}",i)
    }

}

fn get_or_insert(
    mut map: &mut HashMap<u32, String>,
) -> &String {
    // Who needs the entry API?
    polonius!(|map| -> &'polonius String {
        if let Some(v) = map.get(&22) {
            polonius_return!(v);
        }
    });
    map.insert(22, String::from("hi"));
    &map[&22]
}