use std::str::FromStr;
use std::mem;
// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     length:u32,
// }
// // 第一个实验，当一个可变的变量有两个引用
// //一个是可变的，一个是不可变的，能编译成功吗

// impl Rectangle {
//     fn area(self) -> u32{
//         self.width * self.length
//     }
// }

// struct Cacher<T> where T:Fn(u32) -> u32{
//     calculation : T,
//     value : Option<u32>
// }

// enum Coin {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter,
// }
enum En {
    H(i32),
    S(i32,i32,i32,i32),  // 这里占8+4 个字节，但是考虑到对齐这里
    M(),
}

struct Point ( i32 , i32 );
fn main() {
    // println!("Hello, world!");
    // // let mt s = String::from("hello wwwwwww");
    // let m = &s;
    // println!("{}",m);
    // s[2] = "f";
    // let rect = Rectangle{
    //     width:100,
    //     length:100,
    // };
    // println!("{:#?}",rect);
    // println!("{}",rect.area());
    // let coin:Coin = Coin::Dime;
    // let m:u16 = match coin {
    //     Coin::Penny => 1,
    //     Coin::Nickle  => 5,
    //     Coin::Dime    => 10,
    //     Coin::Quarter    => 100,
    // };
    // println!("{}",m);
    // let mut a = String::from("This is a string");
    // // let a[0] = "m";
    // let b = &mut a;
    // // a = Option::None;
    // let b = Option::None;
    // let c = &a;
    // // let b = &mut a;
    // println!("immutable:{:p}",b);
    // // println!("mutable:{:p}",b);
    // println!("origin:{:p}",&a);
    // let m = Some( "hi".to_string());
    println!("the size of is {}",mem::size_of::<En>());
    println!("the align of is {}",mem::align_of::<En>());
    println!("the offset of is {}",mem::<En>());
    offset_of
}



