
struct A{
    a: i32,
    b: i32,
}
impl A{
    fn v(&mut self) -> Vec<&mut i32>{
        vec![&mut self.a, &mut self.b]
    }
}


fn main(){
    let mut a = A{ a: 3, b: 4 };
    let m = a.v();
    {a};
    // *m[0] = 3;
}