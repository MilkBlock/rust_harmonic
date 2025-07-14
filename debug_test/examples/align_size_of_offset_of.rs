use std::mem;
use std::str::FromStr;
enum En {
    H(i32),
    S(i32, i32, i32, i32), // 这里占8+4 个字节，但是考虑到对齐这里
    M(),
}
struct St {
    a: i32,
    b: i32,
}

struct Point(i32, i32);
fn main() {
    println!("the size of is {}", mem::size_of::<En>());
    println!("the align of is {}", mem::align_of::<En>());
    println!("the offset of is {}", mem::offset_of!(St, b));
}
