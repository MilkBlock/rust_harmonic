// use auto_enums::auto_enum;

// #[auto_enum(Iterator)]
fn foo(x: i32) -> Box<dyn Iterator<Item = i32>> {
    match x {
        0 => Box::new(1..10),
        _ => Box::new(vec![5, 10].into_iter()),
    }
}
fn main() {}
