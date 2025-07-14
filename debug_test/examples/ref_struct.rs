struct M;
struct S<'a> {
    a: &'a M,
}
struct V<'a> {
    v: Vec<&'a M>,
}

fn main() {
    let m = M;
    let s1 = S { a: &m };
    let s2 = S { a: &m };

    let (m1, m2, m3) = (M, M, M);
    let v = V {
        v: vec![&m1, &m2, &m3],
    };
}
