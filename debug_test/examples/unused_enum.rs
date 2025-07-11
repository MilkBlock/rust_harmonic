#[allow(unused)]
enum A{
    M {
        a:i32,b:i32
    },
    K {

    }
}
#[allow(unused)]
impl A{
    fn a(){
        let a = A::M { a: 3, b: 5 };
        match a{
            A::M { a, b } => {
            },
            A::K {  } => todo!(),
        }
    }
}
fn main(){

}
