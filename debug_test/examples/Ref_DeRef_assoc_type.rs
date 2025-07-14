struct A;
trait Ref {
    type Ref<'a>: DeRef;
}
trait DeRef {
    type DeRef: Ref;
}
impl Ref for A {
    type Ref<'a> = &'a A;
}
impl<'a> DeRef for <A as Ref>::Ref<'a> {
    type DeRef = A;
}

fn main() {
    let a: <<A as Ref>::Ref<'_> as DeRef>::DeRef = A;
}
