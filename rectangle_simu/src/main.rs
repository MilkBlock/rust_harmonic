#[allow(dead_code)]
#[derive(Clone,Copy,Debug)]
struct Point{ x : f64, y : f64, }
// #[allow(dead_code)]
#[derive(Clone,Copy,Debug)]
#[allow(dead_code)]
struct Rectange<'a>{ top_left : &'a Point, bottom_right : &'a Point, }


fn create_rect<'a:'b , 'b>(p1 :&'b Point, p2: &'b Point) -> 'a Rectange{
    Rectange { top_left: &p1, bottom_right: &p2 }
}
fn main() {
    println!("Hello, world!");
    let s = "hello".split('l');
    let mut s = s.into_iter();
    // for c in s{
    //     println!("{}",c);
    // }
    while let Some(t) = s.next(){
        println!("{t}");
    }
    let r = create_rect();
    println!("{r:?}");
}
