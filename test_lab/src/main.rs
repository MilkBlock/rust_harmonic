use std::mem;
fn longest<'a,T: std::cmp::PartialOrd>(s1 : &'a T, s2 : &'a T) -> &'a T{
    if s1 > s2 {
        s1
    }else{
        s2
    }
}
fn main() {
    // let res;
    // {
    //     let s1 = "hello";
    //     let s2 = "hi";
    //     res = longest_str(s1,s2);
    // }
    // let res;
    // let mut s1 = "hello";
    let mut s2: String = String::from("hello"); //偶对了，String的内部实现本身就是一个u8数组来着
    let s3  =  &mut s2;
    // res = longest(&s1,&s2);
    // unsafe{s1.as_bytes_mut()[0] = 'c' as u8;}
    unsafe{let s3 = s3.as_bytes_mut(); }
    println!("the size of the string is {}",mem::size_of_val(&s2));
    //只要不是把所有权交出去，那么就是 用 s2 
    let mut arr = [1,2,3];
    arr[2] = 4;
    // println!("{}",res);
    println!("{}",arr[2]);
}