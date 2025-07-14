use tuples::{Tuple4Map, Tuple4Map1, TupleAsRef, TupleCombinations2};

fn main() {
    let a = (1, 2, 3, 4);

    a.map(|x| x + 1);
    println!("{:?}", a)
}
