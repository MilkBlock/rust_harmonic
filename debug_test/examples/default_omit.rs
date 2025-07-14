struct M {
    a: i32,
    b: i32,
}

fn main() {
    let m = M {
        a: 3,
        // b: 4,
        ..
    };
}
