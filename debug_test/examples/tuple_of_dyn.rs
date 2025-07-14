struct S1;
struct S2;
struct S3;
trait M {}
impl M for S1 {}
impl M for S2 {}
impl M for S3 {}
fn main() {
    re((&S1, &S2, &S3));
}

fn re(a: (&dyn M, &dyn M, &dyn M)) {}
