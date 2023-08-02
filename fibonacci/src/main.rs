
fn main() {
    let mut a = Some(1);
    let mut b = Some(1);
    while let (Some(i),Some(j)) = (a,b){
        if j<100 {
            println!("{}",i);
            a = Some(j);
            b = Some(i+j);
        }else{
            a = Option::None;
            b = Option::None;
        }
    }
}
