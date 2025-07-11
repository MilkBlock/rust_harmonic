#[derive(Debug)]
struct CTypeStruct{
    test1:u8,
    test2:u8,
    test3:u8,
}
struct TupleStruct(i32,i32,i32);

#[allow(dead_code)]
fn main(){


    fn check1(){
        let struct1 = CTypeStruct{test1:0,test2:255,test3:0};
        println!("{:?}",struct1);
        println!("strcut1.test1 = {}",struct1.test1);
        let struct2 = TupleStruct(1,2,3);
    }
}
