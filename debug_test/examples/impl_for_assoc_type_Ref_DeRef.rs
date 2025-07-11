struct S;
struct S1;
trait M{
    type A;
}
impl M for S{
    type A = i32;
}
impl M for <S as M>::A {
    type A= f64;
}
