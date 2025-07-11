#[derive(Debug)]
struct CTypeStruct{
    test1:u8,
    test2:u8,
    test3:u8,
}
struct TupleStruct(i32,i32,i32);

#[cfg(test)]
#[allow(dead_code)]
mod tests{
   use core::assert_eq;
   use crate::{CTypeStruct, TupleStruct};

   #[test]
   fn check1(){

   let struct1 = CTypeStruct{test1:0,test2:255,test3:0};
   let struct2 = TupleStruct(1,2,3);
   assert_eq!(struct1.test1,struct1.test3);
   }
}
fn main(){

}
