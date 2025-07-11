use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    is_active: bool,
}



#[derive(Serialize, Deserialize, Debug)]
struct Unknown{
    h : HashMap<String,String>
}
fn main() {

    let mut u = User {
        id: 3,
        name: "Hello".to_string(),
        is_active: true,
        // u: todo!(),
    };
    u
    // let m =u.serialize(serializer);
    println!("Hello, world!");
}
