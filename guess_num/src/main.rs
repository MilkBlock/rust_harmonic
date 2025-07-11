use std::prelude::*;
use std::str::FromStr;
use std::io::{self, stdin, BufRead};
// 一个trait 可能要求实现多个方法 因此需要通过添加     impl FromStr for xxx 来单独为这个trait添加所有的方法

fn main() {
    println!("In");
    let lines = stdin().lock().lines();
    for line in lines{
        let s = match line -> String{
            Ok(s) = >s ,
            Err() 
        };
        println!("{}",line);
    }


}
